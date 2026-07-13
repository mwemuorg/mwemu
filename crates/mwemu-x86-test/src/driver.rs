//! Drives a single corpus row through libmwemu: build a clean emulator, apply
//! the initial CPU state, execute exactly one instruction, and diff the result
//! against the hardware oracle.

use iced_x86::{Decoder, DecoderOptions, RflagsBits};
use libmwemu::maps::mem64::Permission;

use crate::corpus::{Case, Outcome, Row};
use crate::keys::{self, Key};

const PAGE: u64 = 0x1000;

/// User-visible arithmetic status flags: CF, PF, AF, ZF, SF, OF. Reserved and
/// system bits (TF, IF, IOPL, ...) are never compared.
const STATUS_FLAGS_MASK: u32 = 0x8d5;

/// Flags a given instruction leaves architecturally undefined, mapped into the
/// `STATUS_FLAGS_MASK` (EFLAGS) bit layout. iced-x86's `RflagsBits` use their own
/// compact bit assignments, so they must be translated. These bits are excluded
/// from the comparison: the oracle recorded whatever the silicon produced there,
/// which is not required to match.
fn undefined_flags(opcode: &[u8], address: u64) -> u32 {
    let mut decoder = Decoder::with_ip(64, opcode, address, DecoderOptions::NONE);
    if !decoder.can_decode() {
        return 0;
    }
    let instr = decoder.decode();
    if instr.is_invalid() {
        return 0;
    }
    let bits = instr.rflags_undefined();
    let mut mask = 0u32;
    if bits & RflagsBits::CF != 0 {
        mask |= 0x1;
    }
    if bits & RflagsBits::PF != 0 {
        mask |= 0x4;
    }
    if bits & RflagsBits::AF != 0 {
        mask |= 0x10;
    }
    if bits & RflagsBits::ZF != 0 {
        mask |= 0x40;
    }
    if bits & RflagsBits::SF != 0 {
        mask |= 0x80;
    }
    if bits & RflagsBits::OF != 0 {
        mask |= 0x800;
    }
    mask
}

pub struct Options {
    pub compare_flags: bool,
}

pub enum Verdict {
    Pass,
    Fail(Vec<String>),
    /// The row referenced state this harness does not model yet (x87/MMX/AVX-512,
    /// control registers, ...), so it is skipped rather than counted as a bug.
    Unsupported(String),
    /// libmwemu decoded nothing or reported the instruction unimplemented. Today
    /// `step()` collapses "unimplemented" and "benign false" into one bool, so
    /// this bucket is a superset of "not implemented" — see the crate README for
    /// the proposed `StepOutcome` change in libmwemu that would separate them.
    NotExecuted,
}

fn width_mask(len: usize) -> u64 {
    if len >= 8 {
        u64::MAX
    } else {
        (1u64 << (len * 8)) - 1
    }
}

fn ensure_mapped(emu: &mut libmwemu::emu::Emu, tag: &str, addr: u64, len: u64, perm: Permission) {
    if len == 0 {
        return;
    }
    if emu.maps.is_mapped(addr) && emu.maps.is_mapped(addr + len - 1) {
        return;
    }
    let base = addr & !(PAGE - 1);
    let end = (addr + len - 1) & !(PAGE - 1);
    let size = end - base + PAGE;
    // May fail if the base page already belongs to another region; in that case
    // the range is already backed and the later write_bytes still lands.
    let _ = emu
        .maps
        .create_map(&format!("{tag}_{base:x}"), base, size, perm);
}

/// Apply one initial binding. Returns `Err(reason)` when the key is unsupported.
fn apply_input(emu: &mut libmwemu::emu::Emu, key: &str, bytes: &[u8]) -> Result<(), String> {
    match keys::classify(key) {
        Key::Scalar(name) => {
            emu.regs_mut().set_by_name(&name, keys::le_u64(bytes));
            Ok(())
        }
        Key::Flags => {
            let value = keys::le_u64(bytes) as u32;
            emu.flags_mut().load(value);
            Ok(())
        }
        Key::Xmm(name) => {
            let low = keys::le_u128(bytes);
            emu.regs_mut().set_xmm_by_name(&name, low);
            // mwemu keeps xmm and ymm in separate fields; mirror into ymm[127:0]
            // so instructions that read the register as ymm see the value.
            let ymm_name = name.replacen("xmm", "ymm", 1);
            let mut buf = [0u8; 32];
            buf[..16].copy_from_slice(&low.to_le_bytes());
            emu.regs_mut()
                .set_ymm_by_name(&ymm_name, libmwemu::regs64::U256::from_little_endian(&buf));
            Ok(())
        }
        Key::Ymm(name) => {
            let mut buf = [0u8; 32];
            let k = bytes.len().min(32);
            buf[..k].copy_from_slice(&bytes[..k]);
            let val = libmwemu::regs64::U256::from_little_endian(&buf);
            emu.regs_mut().set_ymm_by_name(&name, val);
            // Mirror the low 128 bits into the separate xmm field so instructions
            // that read the register as xmm see the architectural value.
            let xmm_name = name.replacen("ymm", "xmm", 1);
            let low = u128::from_le_bytes(buf[0..16].try_into().unwrap());
            emu.regs_mut().set_xmm_by_name(&xmm_name, low);
            Ok(())
        }
        Key::Mem(addr) => {
            ensure_mapped(emu, "mem", addr, bytes.len() as u64, Permission::READ_WRITE);
            if !emu.maps.write_bytes(addr, bytes) {
                return Err(format!("could not write initial memory at 0x{addr:x}"));
            }
            Ok(())
        }
        Key::Unsupported(k) => Err(format!("unsupported state key: {k}")),
    }
}

/// Read back the observed value for an output key, as raw bytes of `len`.
fn read_output(emu: &mut libmwemu::emu::Emu, key: &str, len: usize) -> Result<Vec<u8>, String> {
    match keys::classify(key) {
        Key::Scalar(name) => {
            let value = emu.regs().get_by_name(&name) & width_mask(len);
            Ok(value.to_le_bytes()[..len.min(8)].to_vec())
        }
        Key::Flags => {
            let value = emu.flags_snapshot().dump();
            Ok((value as u64).to_le_bytes()[..len.min(4)].to_vec())
        }
        Key::Xmm(name) => {
            let value = emu.regs().get_xmm_by_name(&name);
            Ok(value.to_le_bytes()[..len.min(16)].to_vec())
        }
        Key::Ymm(name) => {
            let val = emu.regs().get_ymm_by_name(&name);
            let mut buf = [0u8; 32];
            val.to_little_endian(&mut buf);
            Ok(buf[..len.min(32)].to_vec())
        }
        Key::Mem(addr) => match emu.maps.get_mem_by_addr(addr) {
            Some(mem) => Ok(mem.read_bytes(addr, len).to_vec()),
            None => Err(format!("output memory at 0x{addr:x} is not mapped")),
        },
        Key::Unsupported(k) => Err(format!("unsupported state key: {k}")),
    }
}

fn compare(
    key: &str,
    expected: &[u8],
    observed: &[u8],
    compare_flags: bool,
    undefined: u32,
) -> Option<String> {
    match keys::classify(key) {
        Key::Flags => {
            if !compare_flags {
                return None;
            }
            // Compare only the defined status flags for this instruction.
            let mask = STATUS_FLAGS_MASK & !undefined;
            let exp = keys::le_u64(expected) as u32 & mask;
            let obs = keys::le_u64(observed) as u32 & mask;
            if exp == obs {
                None
            } else {
                Some(format!(
                    "flags: expected {exp:#06x} got {obs:#06x} (defined status bits only)"
                ))
            }
        }
        Key::Scalar(name) => {
            let mask = width_mask(expected.len());
            let exp = keys::le_u64(expected) & mask;
            let obs = keys::le_u64(observed) & mask;
            if exp == obs {
                None
            } else {
                Some(format!("{name}: expected {exp:#x} got {obs:#x}"))
            }
        }
        _ => {
            if expected == observed {
                None
            } else {
                Some(format!(
                    "{key}: expected {} got {}",
                    hex(expected),
                    hex(observed)
                ))
            }
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

pub fn run_row(case: &Case, row: &Row, opts: &Options) -> Verdict {
    let mut emu = libmwemu::emu64();

    // Apply the initial register/flag/vector/memory state.
    for (key, bytes) in &row.inputs {
        if let Err(reason) = apply_input(&mut emu, key, bytes) {
            return Verdict::Unsupported(reason);
        }
    }

    // Map the code page and place the instruction bytes.
    ensure_mapped(
        &mut emu,
        "code",
        case.address,
        case.opcode.len() as u64,
        Permission::READ_WRITE_EXECUTE,
    );
    if !emu.maps.write_bytes(case.address, &case.opcode) {
        return Verdict::Unsupported(format!("could not write opcode at 0x{:x}", case.address));
    }

    // Back the stack if the instruction is going to touch it.
    let rsp = emu.regs().rsp;
    if rsp != 0 && !emu.maps.is_mapped(rsp) {
        let base = (rsp & !(PAGE - 1)).saturating_sub(PAGE);
        let _ = emu.maps.create_map(
            &format!("stack_{base:x}"),
            base,
            3 * PAGE,
            Permission::READ_WRITE,
        );
    }

    emu.set_pc(case.address);

    let faults_before = emu.fault_count;
    let executed = emu.step();
    let faulted = emu.fault_count > faults_before || emu.process_terminated;

    match &row.output {
        Outcome::Exception(name) => {
            if faulted {
                Verdict::Pass
            } else if !executed {
                // Could not run at all; treat as a skip, not a semantic failure.
                Verdict::NotExecuted
            } else {
                Verdict::Fail(vec![format!(
                    "expected exception {name} but the instruction executed cleanly"
                )])
            }
        }
        Outcome::State(expected) => {
            if faulted {
                return Verdict::Fail(vec!["unexpected exception / fault during execution".into()]);
            }
            if !executed {
                return Verdict::NotExecuted;
            }
            let undefined = undefined_flags(&case.opcode, case.address);
            let mut diffs = Vec::new();
            for (key, exp_bytes) in expected {
                match read_output(&mut emu, key, exp_bytes.len()) {
                    Ok(observed) => {
                        if let Some(diff) =
                            compare(key, exp_bytes, &observed, opts.compare_flags, undefined)
                        {
                            diffs.push(diff);
                        }
                    }
                    Err(reason) => return Verdict::Unsupported(reason),
                }
            }
            if diffs.is_empty() {
                Verdict::Pass
            } else {
                Verdict::Fail(diffs)
            }
        }
    }
}
