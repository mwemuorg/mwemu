//! Replays the CPU-recorded vectors of `cpu_vectors.rs` in libmwemu.
//!
//! Every vector is a shellcode (`push imm32; popfq; <insn>; pushfq; pop r11;
//! hlt`) run with the input registers, flags and 128-byte data window the real
//! CPU got, and the registers, architecturally defined flags and memory must
//! match what the silicon produced. One test per instruction family so a
//! regression points at the family at fault. Regenerate the table on an Apple
//! Silicon Mac with `scripts/rosetta-difftest/gen_vectors.py`.

use super::cpu_vectors::VECTORS;
use crate::maps::mem64::Permission;
use crate::*;

pub(super) struct Vector {
    pub family: &'static str,
    pub asm: &'static str,
    /// Complete shellcode, hex.
    pub code: &'static str,
    /// Non-zero input registers: (index into rax rbx rcx rdx rsi rdi r8 r9 r10, value).
    pub regs_in: &'static [(usize, u64)],
    pub flags_in: u64,
    /// Registers the CPU changed.
    pub regs_out: &'static [(usize, u64)],
    /// Output flags, already masked.
    pub flags_out: u64,
    pub flags_mask: u64,
    /// 128-byte data window at `BASE` before and after, hex (memory vectors only).
    pub mem_in: &'static str,
    pub mem_out: &'static str,
}

#[allow(clippy::too_many_arguments)]
pub(super) const fn v(
    family: &'static str,
    asm: &'static str,
    code: &'static str,
    regs_in: &'static [(usize, u64)],
    flags_in: u64,
    regs_out: &'static [(usize, u64)],
    flags_out: u64,
    flags_mask: u64,
    mem_in: &'static str,
    mem_out: &'static str,
) -> Vector {
    Vector {
        family,
        asm,
        code,
        regs_in,
        flags_in,
        regs_out,
        flags_out,
        flags_mask,
        mem_in,
        mem_out,
    }
}

const BASE: u64 = 0x3_0000_0000;
const WINDOW: usize = 128;
const SENTINEL: u64 = 0x1122_3344_5566_7788;
const NAMES: [&str; 9] = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10"];

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn set_reg(emu: &mut Emu, idx: usize, value: u64) {
    let regs = emu.regs_mut();
    match idx {
        0 => regs.rax = value,
        1 => regs.rbx = value,
        2 => regs.rcx = value,
        3 => regs.rdx = value,
        4 => regs.rsi = value,
        5 => regs.rdi = value,
        6 => regs.r8 = value,
        7 => regs.r9 = value,
        8 => regs.r10 = value,
        _ => regs.r11 = value,
    }
}

fn get_reg(emu: &Emu, idx: usize) -> u64 {
    let regs = emu.regs();
    match idx {
        0 => regs.rax,
        1 => regs.rbx,
        2 => regs.rcx,
        3 => regs.rdx,
        4 => regs.rsi,
        5 => regs.rdi,
        6 => regs.r8,
        7 => regs.r9,
        8 => regs.r10,
        _ => regs.r11,
    }
}

/// Run one vector and describe every divergence from the CPU.
fn replay(v: &Vector) -> Result<(), String> {
    let mut emu = emu64();
    emu.load_code_bytes(&unhex(v.code));
    if !v.mem_in.is_empty() {
        emu.maps
            .create_map("vec", BASE, 0x10000, Permission::READ_WRITE)
            .expect("data window map");
        emu.maps.write_bytes(BASE, &unhex(v.mem_in));
    }
    let mut expected = [0u64; 9];
    for &(i, value) in v.regs_in {
        set_reg(&mut emu, i, value);
        expected[i] = value;
    }
    for &(i, value) in v.regs_out {
        expected[i] = value;
    }
    set_reg(&mut emu, 9, SENTINEL);
    emu.run(None)
        .map_err(|e| format!("run failed: {}", e.message))?;

    let mut diffs = Vec::new();
    for (i, want) in expected.iter().enumerate() {
        let got = get_reg(&emu, i);
        if got != *want {
            diffs.push(format!("{}: cpu={:#x} mwemu={:#x}", NAMES[i], want, got));
        }
    }
    let flags = get_reg(&emu, 9);
    if flags == SENTINEL {
        diffs.push("did not reach hlt".to_string());
    } else if (flags ^ v.flags_out) & v.flags_mask != 0 {
        diffs.push(format!(
            "flags: cpu={:#x} mwemu={:#x} (mask {:#x})",
            v.flags_out,
            flags & v.flags_mask,
            v.flags_mask
        ));
    }
    if !v.mem_out.is_empty() {
        let got = emu.maps.read_bytes(BASE, WINDOW).to_vec();
        let want = unhex(v.mem_out);
        if let Some(off) = (0..WINDOW).find(|&i| got[i] != want[i]) {
            diffs.push(format!(
                "mem[{off:#x}]: cpu={:02x} mwemu={:02x}",
                want[off], got[off]
            ));
        }
    }
    if diffs.is_empty() {
        Ok(())
    } else {
        Err(diffs.join("; "))
    }
}

fn run_family(family: &str) {
    let mut failures = Vec::new();
    let mut count = 0;
    for vector in VECTORS.iter().filter(|v| v.family == family) {
        count += 1;
        if let Err(e) = replay(vector) {
            failures.push(format!("{} [{}]: {}", vector.asm, vector.code, e));
        }
    }
    assert!(count > 0, "no vectors for family {family}");
    assert!(
        failures.is_empty(),
        "{} of {} `{}` vectors diverge from the CPU:\n  {}",
        failures.len(),
        count,
        family,
        failures.join("\n  ")
    );
}

#[test]
fn cpu_vectors_alu() {
    run_family("alu");
}

#[test]
fn cpu_vectors_bit() {
    run_family("bit");
}

#[test]
fn cpu_vectors_bmi() {
    run_family("bmi");
}

#[test]
fn cpu_vectors_cc() {
    run_family("cc");
}

#[test]
fn cpu_vectors_div() {
    run_family("div");
}

#[test]
fn cpu_vectors_dshift() {
    run_family("dshift");
}

#[test]
fn cpu_vectors_misc() {
    run_family("misc");
}

#[test]
fn cpu_vectors_mov() {
    run_family("mov");
}

#[test]
fn cpu_vectors_mul() {
    run_family("mul");
}

#[test]
fn cpu_vectors_shift() {
    run_family("shift");
}

#[test]
fn cpu_vectors_mem_alu() {
    run_family("mem_alu");
}

#[test]
fn cpu_vectors_mem_bit() {
    run_family("mem_bit");
}

#[test]
fn cpu_vectors_mem_cc() {
    run_family("mem_cc");
}

#[test]
fn cpu_vectors_mem_div() {
    run_family("mem_div");
}

#[test]
fn cpu_vectors_mem_dshift() {
    run_family("mem_dshift");
}

#[test]
fn cpu_vectors_mem_misc() {
    run_family("mem_misc");
}

#[test]
fn cpu_vectors_mem_mov() {
    run_family("mem_mov");
}

#[test]
fn cpu_vectors_mem_mul() {
    run_family("mem_mul");
}

#[test]
fn cpu_vectors_mem_shift() {
    run_family("mem_shift");
}

#[test]
fn cpu_vectors_mem_str() {
    run_family("mem_str");
}

#[test]
fn cpu_vectors_mem_xchg() {
    run_family("mem_xchg");
}
