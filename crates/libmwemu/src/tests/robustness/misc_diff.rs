//! Differential tests for mul/imul (CF/OF) and bt/bsf/bsr (CF/ZF) vs the real
//! x86_64 CPU. These set their flags inline in the handlers, so we emulate the
//! real instruction end-to-end (`load_code_bytes` + `step`) and compare.
//!
//! Flag comparison honors the spec: mul/imul define only CF/OF (SF/ZF/PF/AF
//! undefined); bt defines only CF; bsf/bsr define only ZF (plus the result
//! index when the source is non-zero). div/idiv leave *all* flags undefined, so
//! we don't flag-test them. Host-gated to x86_64.
#![cfg(target_arch = "x86_64")]

use crate::emu::Emu;
use crate::emu64;
use std::arch::asm;

/// Load a single instruction, set up state, single-step, return the emu.
fn run1(code: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    setup(&mut emu);
    emu.step();
    emu
}

fn cf(emu: &mut Emu) -> bool {
    emu.flags_mut().materialize_lazy();
    emu.flags().f_cf
}
fn of(emu: &mut Emu) -> bool {
    emu.flags_mut().materialize_lazy();
    emu.flags().f_of
}
fn zf(emu: &mut Emu) -> bool {
    emu.flags_mut().materialize_lazy();
    emu.flags().f_zf
}

const V8: &[u8] = &[0, 1, 2, 0x0f, 0x10, 0x7f, 0x80, 0x81, 0xff, 0xaa];

// ---- MUL r/m8 (F6 /4 = mul bl): al*bl -> ax, CF=OF = (ah != 0) ----

#[test]
fn mul8_matches_cpu() {
    for &a in V8 {
        for &b in V8 {
            let ax_cpu: u16;
            let rf: u64;
            let axv = a as u16;
            unsafe {
                asm!("mul {b}", "pushfq", "pop {rf}",
                    inout("ax") axv => ax_cpu, b = in(reg_byte) b, rf = out(reg) rf);
            }
            let cpu_cf = rf & 1 != 0;
            let cpu_of = rf & (1 << 11) != 0;

            let mut emu = run1(&[0xf6, 0xe3], |e| {
                e.regs_mut().rax = a as u64;
                e.regs_mut().rbx = b as u64;
            });
            assert_eq!(
                (emu.regs().rax & 0xffff) as u16,
                ax_cpu,
                "mul8({a:#x},{b:#x}) ax"
            );
            assert_eq!(cf(&mut emu), cpu_cf, "mul8({a:#x},{b:#x}) CF");
            assert_eq!(of(&mut emu), cpu_of, "mul8({a:#x},{b:#x}) OF");
        }
    }
}

// ---- IMUL r/m8 (F6 /5 = imul bl): al*bl signed -> ax ----

#[test]
fn imul8_matches_cpu() {
    for &a in V8 {
        for &b in V8 {
            let ax_cpu: u16;
            let rf: u64;
            let axv = a as u16;
            unsafe {
                asm!("imul {b}", "pushfq", "pop {rf}",
                    inout("ax") axv => ax_cpu, b = in(reg_byte) b, rf = out(reg) rf);
            }
            let cpu_cf = rf & 1 != 0;
            let cpu_of = rf & (1 << 11) != 0;

            let mut emu = run1(&[0xf6, 0xeb], |e| {
                e.regs_mut().rax = a as u64;
                e.regs_mut().rbx = b as u64;
            });
            assert_eq!(
                (emu.regs().rax & 0xffff) as u16,
                ax_cpu,
                "imul8({a:#x},{b:#x}) ax"
            );
            assert_eq!(cf(&mut emu), cpu_cf, "imul8({a:#x},{b:#x}) CF");
            assert_eq!(of(&mut emu), cpu_of, "imul8({a:#x},{b:#x}) OF");
        }
    }
}

// ---- BT r/m32, r32 (0F A3 /r = bt eax, ebx): CF = tested bit ----

const V32: &[u32] = &[
    0,
    1,
    0x8000_0000,
    0x7fff_ffff,
    0xffff_ffff,
    0xaaaa_aaaa,
    0x0000_ff00,
];

#[test]
fn bt32_matches_cpu() {
    for &v in V32 {
        for bit in [0u32, 1, 7, 15, 16, 31, 32, 33, 63] {
            let rf: u64;
            unsafe {
                asm!("bt {v:e}, {b:e}", "pushfq", "pop {rf}",
                    v = in(reg) v, b = in(reg) bit, rf = out(reg) rf);
            }
            let cpu_cf = rf & 1 != 0;
            let mut emu = run1(&[0x0f, 0xa3, 0xd8], |e| {
                e.regs_mut().rax = v as u64;
                e.regs_mut().rbx = bit as u64;
            });
            assert_eq!(cf(&mut emu), cpu_cf, "bt32({v:#x}, bit={bit}) CF");
        }
    }
}

// ---- BSF r32, r/m32 (0F BC /r = bsf eax, ebx): ZF=(src==0), else eax=index ----

#[test]
fn bsf32_matches_cpu() {
    for &v in V32 {
        let (dst_cpu, rf): (u32, u64);
        let d: u32 = 0xdead_beef;
        unsafe {
            asm!("bsf {d:e}, {s:e}", "pushfq", "pop {rf}",
                d = inout(reg) d => dst_cpu, s = in(reg) v, rf = out(reg) rf);
        }
        let cpu_zf = rf & (1 << 6) != 0;
        let mut emu = run1(&[0x0f, 0xbc, 0xc3], |e| {
            e.regs_mut().rax = 0xdead_beef;
            e.regs_mut().rbx = v as u64;
        });
        assert_eq!(zf(&mut emu), cpu_zf, "bsf32({v:#x}) ZF");
        if v != 0 {
            assert_eq!(
                (emu.regs().rax & 0xffff_ffff) as u32,
                dst_cpu,
                "bsf32({v:#x}) index"
            );
        }
    }
}

// ---- BSR r32, r/m32 (0F BD /r): ZF=(src==0), else eax=index of top set bit ----

#[test]
fn bsr32_matches_cpu() {
    for &v in V32 {
        let (dst_cpu, rf): (u32, u64);
        let d: u32 = 0xdead_beef;
        unsafe {
            asm!("bsr {d:e}, {s:e}", "pushfq", "pop {rf}",
                d = inout(reg) d => dst_cpu, s = in(reg) v, rf = out(reg) rf);
        }
        let cpu_zf = rf & (1 << 6) != 0;
        let mut emu = run1(&[0x0f, 0xbd, 0xc3], |e| {
            e.regs_mut().rax = 0xdead_beef;
            e.regs_mut().rbx = v as u64;
        });
        assert_eq!(zf(&mut emu), cpu_zf, "bsr32({v:#x}) ZF");
        if v != 0 {
            assert_eq!(
                (emu.regs().rax & 0xffff_ffff) as u32,
                dst_cpu,
                "bsr32({v:#x}) index"
            );
        }
    }
}
