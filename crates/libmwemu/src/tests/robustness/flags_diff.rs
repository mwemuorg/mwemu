//! Differential flag testing against the real x86_64 CPU — the ground truth.
//!
//! For each arithmetic helper in `flags.rs` (the exact functions the instruction
//! handlers call, e.g. `add` → `flags.add8/16/32/64`), we run the *real* machine
//! instruction over a matrix of edge-case operands, capture RFLAGS, and compare
//! against what mwemu computes. A mismatch is a **certain** bug (the CPU is the
//! oracle) with an exact reproducing input; agreement proves correctness.
//!
//! Host-gated to x86_64 (the CI ubuntu runner); a no-op elsewhere (macOS ARM).
#![cfg(target_arch = "x86_64")]

use crate::arch::x86::flags::Flags;
use std::arch::asm;

/// (CF, PF, AF, ZF, SF, OF) extracted from RFLAGS.
type F6 = (bool, bool, bool, bool, bool, bool);

#[inline]
fn rflags_bits(rf: u64) -> F6 {
    (
        rf & (1 << 0) != 0,  // CF
        rf & (1 << 2) != 0,  // PF
        rf & (1 << 4) != 0,  // AF
        rf & (1 << 6) != 0,  // ZF
        rf & (1 << 7) != 0,  // SF
        rf & (1 << 11) != 0, // OF
    )
}

/// Edge-case operand set: zero, one, nibble/byte carry boundaries, sign
/// boundaries, all-ones, alternating bits.
const VALS8: &[u8] = &[
    0x00, 0x01, 0x02, 0x0f, 0x10, 0x7f, 0x80, 0x81, 0xfe, 0xff, 0x40, 0xc0, 0xaa, 0x55,
];

// ---- real CPU oracles ----

fn cpu_add8(a: u8, b: u8) -> (u8, F6) {
    let res: u8;
    let rf: u64;
    unsafe {
        asm!(
            "add {a}, {b}",
            "pushfq",
            "pop {rf}",
            a = inout(reg_byte) a => res,
            b = in(reg_byte) b,
            rf = out(reg) rf,
        );
    }
    (res, rflags_bits(rf))
}

fn cpu_sub8(a: u8, b: u8) -> (u8, F6) {
    let res: u8;
    let rf: u64;
    unsafe {
        asm!(
            "sub {a}, {b}",
            "pushfq",
            "pop {rf}",
            a = inout(reg_byte) a => res,
            b = in(reg_byte) b,
            rf = out(reg) rf,
        );
    }
    (res, rflags_bits(rf))
}

// ---- mwemu computations (materialized, exactly as the emulator reads them) ----

fn mwemu_flags(f: &mut Flags) -> F6 {
    f.materialize_lazy();
    (f.f_cf, f.f_pf, f.f_af, f.f_zf, f.f_sf, f.f_of)
}

fn assert_match(op: &str, a: u8, b: u8, cpu: (u8, F6), mwemu: (u8, F6)) {
    let names = ["CF", "PF", "AF", "ZF", "SF", "OF"];
    assert_eq!(
        cpu.0, mwemu.0,
        "{op}8({a:#04x},{b:#04x}) result: cpu={:#04x} mwemu={:#04x}",
        cpu.0, mwemu.0
    );
    let cf = [cpu.1.0, cpu.1.1, cpu.1.2, cpu.1.3, cpu.1.4, cpu.1.5];
    let mf = [
        mwemu.1.0, mwemu.1.1, mwemu.1.2, mwemu.1.3, mwemu.1.4, mwemu.1.5,
    ];
    for i in 0..6 {
        assert_eq!(
            cf[i], mf[i],
            "{op}8({a:#04x},{b:#04x}) flag {}: cpu={} mwemu={}",
            names[i], cf[i], mf[i]
        );
    }
}

#[test]
fn add8_matches_cpu() {
    for &a in VALS8 {
        for &b in VALS8 {
            let cpu = cpu_add8(a, b);
            let mut f = Flags::new();
            let res = f.add8(a, b, false, false) as u8;
            assert_match("add", a, b, cpu, (res, mwemu_flags(&mut f)));
        }
    }
}

#[test]
fn sub8_matches_cpu() {
    for &a in VALS8 {
        for &b in VALS8 {
            let cpu = cpu_sub8(a, b);
            let mut f = Flags::new();
            let res = f.sub8(a as u64, b as u64) as u8;
            assert_match("sub", a, b, cpu, (res, mwemu_flags(&mut f)));
        }
    }
}

// ---- wider add/sub (16/32/64) ----

const VALS16: &[u16] = &[
    0, 1, 0xff, 0x100, 0x7fff, 0x8000, 0x8001, 0xfffe, 0xffff, 0xaaaa, 0x5555,
];
const VALS32: &[u32] = &[
    0,
    1,
    0xffff,
    0x1_0000,
    0x7fff_ffff,
    0x8000_0000,
    0x8000_0001,
    0xffff_fffe,
    0xffff_ffff,
    0xaaaa_aaaa,
];
const VALS64: &[u64] = &[
    0,
    1,
    0xffff_ffff,
    0x1_0000_0000,
    0x7fff_ffff_ffff_ffff,
    0x8000_0000_0000_0000,
    0x8000_0000_0000_0001,
    0xffff_ffff_ffff_fffe,
    0xffff_ffff_ffff_ffff,
];

macro_rules! diff_test {
    ($test:ident, $op:literal, $vals:ident, $ty:ty, $modif:literal, $insn:literal, $mwemu:expr) => {
        #[test]
        fn $test() {
            for &a in $vals {
                for &b in $vals {
                    let (res_cpu, rf): ($ty, u64);
                    unsafe {
                        asm!(
                            concat!($insn, " {a:", $modif, "}, {b:", $modif, "}"),
                            "pushfq",
                            "pop {rf}",
                            a = inout(reg) a => res_cpu,
                            b = in(reg) b,
                            rf = out(reg) rf,
                        );
                    }
                    let mut f = Flags::new();
                    let res_m: $ty = ($mwemu)(&mut f, a, b) as $ty;
                    let mf = mwemu_flags(&mut f);
                    assert_eq!(res_cpu, res_m, concat!($op, "({:#x},{:#x}) result"), a, b);
                    let cb = rflags_bits(rf);
                    let names = ["CF", "PF", "AF", "ZF", "SF", "OF"];
                    let cf = [cb.0, cb.1, cb.2, cb.3, cb.4, cb.5];
                    let mv = [mf.0, mf.1, mf.2, mf.3, mf.4, mf.5];
                    for i in 0..6 {
                        assert_eq!(cf[i], mv[i], concat!($op, "({:#x},{:#x}) flag {}"), a, b, names[i]);
                    }
                }
            }
        }
    };
}

diff_test!(
    add16_matches_cpu,
    "add16",
    VALS16,
    u16,
    "x",
    "add",
    |f: &mut Flags, a, b| f.add16(a, b, false, false)
);
diff_test!(
    add32_matches_cpu,
    "add32",
    VALS32,
    u32,
    "e",
    "add",
    |f: &mut Flags, a, b| f.add32(a, b, false, false)
);
diff_test!(
    add64_matches_cpu,
    "add64",
    VALS64,
    u64,
    "r",
    "add",
    |f: &mut Flags, a, b| f.add64(a, b, false, false)
);
diff_test!(
    sub16_matches_cpu,
    "sub16",
    VALS16,
    u16,
    "x",
    "sub",
    |f: &mut Flags, a, b| f.sub16(a as u64, b as u64)
);
diff_test!(
    sub32_matches_cpu,
    "sub32",
    VALS32,
    u32,
    "e",
    "sub",
    |f: &mut Flags, a, b| f.sub32(a as u64, b as u64)
);
diff_test!(
    sub64_matches_cpu,
    "sub64",
    VALS64,
    u64,
    "r",
    "sub",
    |f: &mut Flags, a, b| f.sub64(a as u64, b as u64)
);

// ---- adc / sbb (carry-in path — a classic bug spot) ----

#[test]
fn adc8_matches_cpu() {
    for &carry in &[false, true] {
        for &a in VALS8 {
            for &b in VALS8 {
                let (res_cpu, rf): (u8, u64);
                unsafe {
                    asm!(
                        "bt {cin}, 0", // set CF = carry-in bit 0
                        "adc {a}, {b}",
                        "pushfq",
                        "pop {rf}",
                        cin = in(reg) carry as u64,
                        a = inout(reg_byte) a => res_cpu,
                        b = in(reg_byte) b,
                        rf = out(reg) rf,
                    );
                }
                let mut f = Flags::new();
                let res_m = f.add8(a, b, carry, true) as u8;
                assert_match(
                    "adc",
                    a,
                    b,
                    (res_cpu, rflags_bits(rf)),
                    (res_m, mwemu_flags(&mut f)),
                );
            }
        }
    }
}

#[test]
fn sbb8_matches_cpu() {
    for &borrow in &[false, true] {
        for &a in VALS8 {
            for &b in VALS8 {
                let (res_cpu, rf): (u8, u64);
                unsafe {
                    asm!(
                        "bt {cin}, 0",
                        "sbb {a}, {b}",
                        "pushfq",
                        "pop {rf}",
                        cin = in(reg) borrow as u64,
                        a = inout(reg_byte) a => res_cpu,
                        b = in(reg_byte) b,
                        rf = out(reg) rf,
                    );
                }
                let mut f = Flags::new();
                let res_m = f.sub8_borrow(a as u64, b as u64, borrow) as u8;
                assert_match(
                    "sbb",
                    a,
                    b,
                    (res_cpu, rflags_bits(rf)),
                    (res_m, mwemu_flags(&mut f)),
                );
            }
        }
    }
}

// ---- inc / dec (must PRESERVE CF) / neg ----

#[test]
fn inc8_matches_cpu() {
    for &a in VALS8 {
        let (res_cpu, rf): (u8, u64);
        unsafe {
            asm!("clc", "inc {a}", "pushfq", "pop {rf}", a = inout(reg_byte) a => res_cpu, rf = out(reg) rf);
        }
        let mut f = Flags::new(); // CF starts clear, like clc
        let res_m = f.inc8(a as u64) as u8;
        assert_match(
            "inc",
            a,
            0,
            (res_cpu, rflags_bits(rf)),
            (res_m, mwemu_flags(&mut f)),
        );
    }
}

#[test]
fn dec8_matches_cpu() {
    for &a in VALS8 {
        let (res_cpu, rf): (u8, u64);
        unsafe {
            asm!("clc", "dec {a}", "pushfq", "pop {rf}", a = inout(reg_byte) a => res_cpu, rf = out(reg) rf);
        }
        let mut f = Flags::new();
        let res_m = f.dec8(a as u64) as u8;
        assert_match(
            "dec",
            a,
            0,
            (res_cpu, rflags_bits(rf)),
            (res_m, mwemu_flags(&mut f)),
        );
    }
}

#[test]
fn neg8_matches_cpu() {
    for &a in VALS8 {
        let (res_cpu, rf): (u8, u64);
        unsafe {
            asm!("neg {a}", "pushfq", "pop {rf}", a = inout(reg_byte) a => res_cpu, rf = out(reg) rf);
        }
        let mut f = Flags::new();
        let res_m = f.neg8(a as u64) as u8;
        assert_match(
            "neg",
            a,
            0,
            (res_cpu, rflags_bits(rf)),
            (res_m, mwemu_flags(&mut f)),
        );
    }
}

macro_rules! neg_test {
    ($test:ident, $vals:ident, $ty:ty, $modif:literal, $mwemu:ident) => {
        #[test]
        fn $test() {
            for &a in $vals {
                let (res_cpu, rf): ($ty, u64);
                unsafe {
                    asm!(concat!("neg {a:", $modif, "}"), "pushfq", "pop {rf}",
                        a = inout(reg) a => res_cpu, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                let res_m: $ty = f.$mwemu(a as u64) as $ty;
                let mf = mwemu_flags(&mut f);
                assert_eq!(res_cpu, res_m, concat!(stringify!($test), " {:#x} result"), a);
                let cb = rflags_bits(rf);
                let names = ["CF", "PF", "AF", "ZF", "SF", "OF"];
                let cf = [cb.0, cb.1, cb.2, cb.3, cb.4, cb.5];
                let mv = [mf.0, mf.1, mf.2, mf.3, mf.4, mf.5];
                for i in 0..6 {
                    assert_eq!(cf[i], mv[i], concat!(stringify!($test), " {:#x} flag {}"), a, names[i]);
                }
            }
        }
    };
}

neg_test!(neg16_matches_cpu, VALS16, u16, "x", neg16);
neg_test!(neg32_matches_cpu, VALS32, u32, "e", neg32);
neg_test!(neg64_matches_cpu, VALS64, u64, "r", neg64);
