//! Drives a single corpus row through libmwemu: build a clean emulator, apply
//! the initial CPU state, execute exactly one instruction, and diff the result
//! against the hardware oracle.

use libmwemu::maps::mem64::Permission;

use crate::corpus::{Case, Outcome, Row};
use crate::keys::{self, Key};

const PAGE: u64 = 0x1000;

/// User-visible arithmetic status flags: CF, PF, AF, ZF, SF, OF. Reserved and
/// system bits (TF, IF, IOPL, ...) are never compared.
const STATUS_FLAGS_MASK: u32 = 0x8d5;

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
            emu.regs_mut().set_xmm_by_name(&name, keys::le_u128(bytes));
            Ok(())
        }
        Key::Ymm(name) => Err(format!("ymm not modelled yet: {name}")),
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
        Key::Ymm(name) => Err(format!("ymm not modelled yet: {name}")),
        Key::Mem(addr) => match emu.maps.get_mem_by_addr(addr) {
            Some(mem) => Ok(mem.read_bytes(addr, len).to_vec()),
            None => Err(format!("output memory at 0x{addr:x} is not mapped")),
        },
        Key::Unsupported(k) => Err(format!("unsupported state key: {k}")),
    }
}

fn compare(key: &str, expected: &[u8], observed: &[u8], compare_flags: bool) -> Option<String> {
    match keys::classify(key) {
        Key::Flags => {
            if !compare_flags {
                return None;
            }
            let exp = keys::le_u64(expected) as u32 & STATUS_FLAGS_MASK;
            let obs = keys::le_u64(observed) as u32 & STATUS_FLAGS_MASK;
            if exp == obs {
                None
            } else {
                Some(format!(
                    "flags: expected {exp:#06x} got {obs:#06x} (status bits only)"
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
            let mut diffs = Vec::new();
            for (key, exp_bytes) in expected {
                match read_output(&mut emu, key, exp_bytes.len()) {
                    Ok(observed) => {
                        if let Some(diff) = compare(key, exp_bytes, &observed, opts.compare_flags) {
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
