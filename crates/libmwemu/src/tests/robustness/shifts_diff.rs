//! Differential shift flag testing against the SDM reference model in
//! `oracle.rs`.
//!
//! Shifts are the classic flag-bug hotspot: count masking (5-bit for 8/16/32,
//! 6-bit for 64), CF = last bit out, and OF that is *only architecturally
//! defined for count==1*. We therefore compare CF/SF/ZF/PF whenever the count
//! is non-zero, OF only when count==1, and never AF (undefined for shifts).

use super::oracle;
use crate::arch::x86::flags::Flags;

const COUNTS: &[u8] = &[0, 1, 2, 7, 8, 15, 16, 17, 31];

/// Compare mwemu's post-op result and flags against the oracle, honoring which
/// flags a shift actually defines for this `count`.
fn check(op: &str, val: u64, count: u8, oracle: (u64, u64), res_m: u64, f: &mut Flags) {
    assert_eq!(oracle.0, res_m, "{op}({val:#x},{count}) result");
    f.materialize_lazy();
    if count == 0 {
        return; // shifts by 0 leave flags untouched — nothing to compare
    }
    let rf = oracle.1;
    assert_eq!(f.f_cf, rf & oracle::CF != 0, "{op}({val:#x}, {count}) CF");
    assert_eq!(f.f_zf, rf & oracle::ZF != 0, "{op}({val:#x}, {count}) ZF");
    assert_eq!(f.f_sf, rf & oracle::SF != 0, "{op}({val:#x}, {count}) SF");
    assert_eq!(f.f_pf, rf & oracle::PF != 0, "{op}({val:#x}, {count}) PF");
    if count == 1 {
        assert_eq!(f.f_of, rf & oracle::OF != 0, "{op}({val:#x}, {count}) OF");
    }
}

/// One single-operand shift (shl/shr/sar) at one width over `vals × COUNTS`.
fn shift_test(
    op: &str,
    w: u32,
    vals: &[u64],
    oracle: fn(u64, u8, u32) -> (u64, u64),
    mwemu: fn(&mut Flags, u64, u64) -> u64,
) {
    for &v in vals {
        for &c in COUNTS {
            let mut f = Flags::new();
            let res_m = mwemu(&mut f, v, c as u64) & (u64::MAX >> (64 - w));
            check(op, v, c, oracle(v, c, w), res_m, &mut f);
        }
    }
}

// ---- 8-bit variable-count shifts ----

const VALS8: &[u64] = &[0x00, 0x01, 0x80, 0x81, 0xff, 0x7f, 0xaa, 0x55, 0x0f, 0xf0];

#[test]
fn shl8_matches_oracle() {
    shift_test("shl8", 8, VALS8, oracle::shl, Flags::shl2p8);
}

#[test]
fn shr8_matches_oracle() {
    shift_test("shr8", 8, VALS8, oracle::shr, Flags::shr2p8);
}

#[test]
fn sar8_matches_oracle() {
    shift_test("sar8", 8, VALS8, oracle::sar, Flags::sar2p8);
}

// ---- 16-bit variable-count shifts (exercises the sar count>=width sign path) ----

const VALS16: &[u64] = &[0, 1, 0x8000, 0x7fff, 0xffff, 0xaaaa, 0x00ff, 0x0080];

#[test]
fn shl16_matches_oracle() {
    shift_test("shl16", 16, VALS16, oracle::shl, Flags::shl2p16);
}

#[test]
fn shr16_matches_oracle() {
    shift_test("shr16", 16, VALS16, oracle::shr, Flags::shr2p16);
}

#[test]
fn sar16_matches_oracle() {
    shift_test("sar16", 16, VALS16, oracle::sar, Flags::sar2p16);
}

// ---- 32-bit variable-count shifts ----

const VALS32: &[u64] = &[
    0,
    1,
    0x8000_0000,
    0x7fff_ffff,
    0xffff_ffff,
    0xaaaa_aaaa,
    0x0000_00ff,
];

#[test]
fn shl32_matches_oracle() {
    shift_test("shl32", 32, VALS32, oracle::shl, Flags::shl2p32);
}

#[test]
fn shr32_matches_oracle() {
    shift_test("shr32", 32, VALS32, oracle::shr, Flags::shr2p32);
}

#[test]
fn sar32_matches_oracle() {
    shift_test("sar32", 32, VALS32, oracle::sar, Flags::sar2p32);
}

// ---- shld / shrd (double-precision shifts) ----
// For 32/64-bit, every masked count is architecturally defined.

const DBL32: &[(u64, u64)] = &[
    (0, 0),
    (1, 0),
    (0x8000_0000, 0xffff_ffff),
    (0xdead_beef, 0x1234_5678),
    (0xffff_ffff, 0),
    (0, 0xffff_ffff),
    (0x7fff_ffff, 0x8000_0000),
    (0xaaaa_aaaa, 0x5555_5555),
];

const DBL64: &[(u64, u64)] = &[
    (0, 0),
    (1, 0),
    (0x8000_0000_0000_0000, u64::MAX),
    (0xdead_beef_cafe_babe, 0x0123_4567_89ab_cdef),
    (u64::MAX, 0),
    (0, u64::MAX),
];
const COUNTS64: &[u8] = &[0, 1, 2, 31, 32, 33, 63];

/// One double-precision shift (shld/shrd) at one width over `pairs × counts`.
fn dbl_shift_test(
    op: &str,
    w: u32,
    pairs: &[(u64, u64)],
    counts: &[u8],
    oracle: fn(u64, u64, u8, u32) -> (u64, u64),
    mwemu: fn(&mut Flags, u64, u64, u64, u32) -> u64,
) {
    for &(v0, v1) in pairs {
        for &c in counts {
            let mut f = Flags::new();
            let res_m = mwemu(&mut f, v0, v1, c as u64, w) & (u64::MAX >> (64 - w));
            check(op, v0, c, oracle(v0, v1, c, w), res_m, &mut f);
        }
    }
}

#[test]
fn shld32_matches_oracle() {
    dbl_shift_test("shld32", 32, DBL32, COUNTS, oracle::shld, Flags::shld);
}

#[test]
fn shrd32_matches_oracle() {
    dbl_shift_test("shrd32", 32, DBL32, COUNTS, oracle::shrd, Flags::shrd);
}

#[test]
fn shld64_matches_oracle() {
    dbl_shift_test("shld64", 64, DBL64, COUNTS64, oracle::shld, Flags::shld);
}

#[test]
fn shrd64_matches_oracle() {
    dbl_shift_test("shrd64", 64, DBL64, COUNTS64, oracle::shrd, Flags::shrd);
}
