//! Differential flag testing for the add/sub family against the SDM reference
//! model in `oracle.rs`.
//!
//! For each arithmetic helper in `flags.rs` (the exact functions the instruction
//! handlers call, e.g. `add` → `flags.add8/16/32/64`), we run the reference
//! model over a matrix of edge-case operands and compare result and flags with
//! what mwemu computes. A mismatch is a bug in one of the two with an exact
//! reproducing input; the oracle shares no code with the emulator.

use super::oracle;
use crate::arch::x86::flags::Flags;

/// (CF, PF, AF, ZF, SF, OF) extracted from RFLAGS.
type F6 = (bool, bool, bool, bool, bool, bool);

#[inline]
fn rflags_bits(rf: u64) -> F6 {
    (
        rf & oracle::CF != 0,
        rf & oracle::PF != 0,
        rf & oracle::AF != 0,
        rf & oracle::ZF != 0,
        rf & oracle::SF != 0,
        rf & oracle::OF != 0,
    )
}

/// Edge-case operand set: zero, one, nibble/byte carry boundaries, sign
/// boundaries, all-ones, alternating bits.
const VALS8: &[u8] = &[
    0x00, 0x01, 0x02, 0x0f, 0x10, 0x7f, 0x80, 0x81, 0xfe, 0xff, 0x40, 0xc0, 0xaa, 0x55,
];

// ---- mwemu computations (materialized, exactly as the emulator reads them) ----

fn mwemu_flags(f: &mut Flags) -> F6 {
    f.materialize_lazy();
    (f.f_cf, f.f_pf, f.f_af, f.f_zf, f.f_sf, f.f_of)
}

fn assert_match(op: &str, a: u64, b: u64, oracle: (u64, u64), mwemu: (u64, F6)) {
    let names = ["CF", "PF", "AF", "ZF", "SF", "OF"];
    assert_eq!(
        oracle.0, mwemu.0,
        "{op}({a:#x},{b:#x}) result: oracle={:#x} mwemu={:#x}",
        oracle.0, mwemu.0
    );
    let o = rflags_bits(oracle.1);
    let of = [o.0, o.1, o.2, o.3, o.4, o.5];
    let mf = [
        mwemu.1.0, mwemu.1.1, mwemu.1.2, mwemu.1.3, mwemu.1.4, mwemu.1.5,
    ];
    for i in 0..6 {
        assert_eq!(
            of[i], mf[i],
            "{op}({a:#x},{b:#x}) flag {}: oracle={} mwemu={}",
            names[i], of[i], mf[i]
        );
    }
}

#[test]
fn add8_matches_oracle() {
    for &a in VALS8 {
        for &b in VALS8 {
            let mut f = Flags::new();
            let res = f.add8(a, b, false, false) & 0xff;
            assert_match(
                "add8",
                a as u64,
                b as u64,
                oracle::add(a as u64, b as u64, false, 8),
                (res, mwemu_flags(&mut f)),
            );
        }
    }
}

#[test]
fn sub8_matches_oracle() {
    for &a in VALS8 {
        for &b in VALS8 {
            let mut f = Flags::new();
            let res = f.sub8(a as u64, b as u64) & 0xff;
            assert_match(
                "sub8",
                a as u64,
                b as u64,
                oracle::sub(a as u64, b as u64, false, 8),
                (res, mwemu_flags(&mut f)),
            );
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
    ($test:ident, $op:literal, $vals:ident, $w:literal, $oracle:expr, $mwemu:expr) => {
        #[test]
        fn $test() {
            for &a in $vals {
                for &b in $vals {
                    let mut f = Flags::new();
                    let res_m = ($mwemu)(&mut f, a, b) & (u64::MAX >> (64 - $w));
                    let mf = mwemu_flags(&mut f);
                    assert_match(
                        $op,
                        a as u64,
                        b as u64,
                        ($oracle)(a as u64, b as u64),
                        (res_m, mf),
                    );
                }
            }
        }
    };
}

diff_test!(
    add16_matches_oracle,
    "add16",
    VALS16,
    16,
    |a, b| oracle::add(a, b, false, 16),
    |f: &mut Flags, a, b| f.add16(a, b, false, false)
);
diff_test!(
    add32_matches_oracle,
    "add32",
    VALS32,
    32,
    |a, b| oracle::add(a, b, false, 32),
    |f: &mut Flags, a, b| f.add32(a, b, false, false)
);
diff_test!(
    add64_matches_oracle,
    "add64",
    VALS64,
    64,
    |a, b| oracle::add(a, b, false, 64),
    |f: &mut Flags, a, b| f.add64(a, b, false, false)
);
diff_test!(
    sub16_matches_oracle,
    "sub16",
    VALS16,
    16,
    |a, b| oracle::sub(a, b, false, 16),
    |f: &mut Flags, a, b| f.sub16(a as u64, b as u64)
);
diff_test!(
    sub32_matches_oracle,
    "sub32",
    VALS32,
    32,
    |a, b| oracle::sub(a, b, false, 32),
    |f: &mut Flags, a, b| f.sub32(a as u64, b as u64)
);
diff_test!(
    sub64_matches_oracle,
    "sub64",
    VALS64,
    64,
    |a, b| oracle::sub(a, b, false, 64),
    |f: &mut Flags, a, b| f.sub64(a, b)
);

// ---- adc / sbb (carry-in path — a classic bug spot) ----

#[test]
fn adc8_matches_oracle() {
    for &carry in &[false, true] {
        for &a in VALS8 {
            for &b in VALS8 {
                let mut f = Flags::new();
                let res_m = f.add8(a, b, carry, true) & 0xff;
                assert_match(
                    "adc8",
                    a as u64,
                    b as u64,
                    oracle::add(a as u64, b as u64, carry, 8),
                    (res_m, mwemu_flags(&mut f)),
                );
            }
        }
    }
}

#[test]
fn sbb8_matches_oracle() {
    for &borrow in &[false, true] {
        for &a in VALS8 {
            for &b in VALS8 {
                let mut f = Flags::new();
                let res_m = f.sub8_borrow(a as u64, b as u64, borrow) & 0xff;
                assert_match(
                    "sbb8",
                    a as u64,
                    b as u64,
                    oracle::sub(a as u64, b as u64, borrow, 8),
                    (res_m, mwemu_flags(&mut f)),
                );
            }
        }
    }
}

// ---- inc / dec (must PRESERVE CF) / neg ----

#[test]
fn inc8_matches_oracle() {
    for &a in VALS8 {
        let mut f = Flags::new(); // CF starts clear, like clc
        let res_m = f.inc8(a as u64) & 0xff;
        assert_match(
            "inc8",
            a as u64,
            0,
            oracle::inc(a as u64, false, 8),
            (res_m, mwemu_flags(&mut f)),
        );
    }
}

#[test]
fn dec8_matches_oracle() {
    for &a in VALS8 {
        let mut f = Flags::new();
        let res_m = f.dec8(a as u64) & 0xff;
        assert_match(
            "dec8",
            a as u64,
            0,
            oracle::dec(a as u64, false, 8),
            (res_m, mwemu_flags(&mut f)),
        );
    }
}

macro_rules! neg_test {
    ($test:ident, $vals:ident, $w:literal, $mwemu:ident) => {
        #[test]
        fn $test() {
            for &a in $vals {
                let mut f = Flags::new();
                let res_m = f.$mwemu(a as u64) & (u64::MAX >> (64 - $w));
                assert_match(
                    stringify!($mwemu),
                    a as u64,
                    0,
                    oracle::neg(a as u64, $w),
                    (res_m, mwemu_flags(&mut f)),
                );
            }
        }
    };
}

neg_test!(neg8_matches_oracle, VALS8, 8, neg8);
neg_test!(neg16_matches_oracle, VALS16, 16, neg16);
neg_test!(neg32_matches_oracle, VALS32, 32, neg32);
neg_test!(neg64_matches_oracle, VALS64, 64, neg64);
