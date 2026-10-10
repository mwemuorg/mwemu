//! Differential rotate flag testing against the SDM reference model in
//! `oracle.rs`.
//!
//! Unlike shifts, rotates (ROL/ROR/RCL/RCR) affect **only CF and OF** — SF/ZF/PF/
//! AF are left untouched — so we compare CF (whenever the masked count is
//! non-zero) and OF (only for count==1), and nothing else. RCL/RCR rotate through
//! CF, so we seed the carry-in on both sides.

use super::oracle;
use crate::arch::x86::flags::Flags;

const COUNTS: &[u8] = &[0, 1, 2, 7, 8, 9, 15, 16, 17, 31];

const VALS8: &[u64] = &[0x00, 0x01, 0x80, 0x81, 0xff, 0x7f, 0xaa, 0x55, 0x0f];
const VALS16: &[u64] = &[0, 1, 0x8000, 0x7fff, 0xffff, 0xaaaa, 0x00ff, 0x0080];
const VALS32: &[u64] = &[
    0,
    1,
    0x8000_0000,
    0x7fff_ffff,
    0xffff_ffff,
    0xaaaa_aaaa,
    0xdead_beef,
];

fn check_rot(
    op: &str,
    val: u64,
    count: u8,
    cin: bool,
    oracle: (u64, u64),
    res_m: u64,
    f: &mut Flags,
) {
    assert_eq!(oracle.0, res_m, "{op}({val:#x},{count},cf={cin}) result");
    f.materialize_lazy();
    if count & 0x1f == 0 {
        return; // masked count 0 → rotate leaves flags untouched
    }
    assert_eq!(
        f.f_cf,
        oracle.1 & oracle::CF != 0,
        "{op}({val:#x}, {count}, cf={cin}) CF"
    );
    if count == 1 {
        assert_eq!(
            f.f_of,
            oracle.1 & oracle::OF != 0,
            "{op}({val:#x}, {count}, cf={cin}) OF"
        );
    }
}

/// ROL / ROR: no carry-in, so `cin` is always false on both sides.
fn rot_test(
    op: &str,
    w: u32,
    vals: &[u64],
    oracle: fn(u64, u8, u32) -> (u64, u64),
    mwemu: fn(&mut Flags, u64, u64, u32) -> u64,
) {
    for &v in vals {
        for &c in COUNTS {
            let mut f = Flags::new();
            let res_m = mwemu(&mut f, v, c as u64, w) & (u64::MAX >> (64 - w));
            check_rot(op, v, c, false, oracle(v, c, w), res_m, &mut f);
        }
    }
}

/// RCL / RCR: rotate through carry, so every case runs with CF seeded 0 and 1.
fn rc_test(
    op: &str,
    w: u32,
    vals: &[u64],
    oracle: fn(u64, u8, u32, bool) -> (u64, u64),
    mwemu: fn(&mut Flags, u64, u64, u32) -> u64,
) {
    for &cin in &[false, true] {
        for &v in vals {
            for &c in COUNTS {
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = mwemu(&mut f, v, c as u64, w) & (u64::MAX >> (64 - w));
                check_rot(op, v, c, cin, oracle(v, c, w, cin), res_m, &mut f);
            }
        }
    }
}

// ---- ROL / ROR (no carry) ----

#[test]
fn rol8_matches_oracle() {
    rot_test("rol8", 8, VALS8, oracle::rol, Flags::rol);
}

#[test]
fn ror8_matches_oracle() {
    rot_test("ror8", 8, VALS8, oracle::ror, Flags::ror);
}

#[test]
fn rol16_matches_oracle() {
    rot_test("rol16", 16, VALS16, oracle::rol, Flags::rol);
}

#[test]
fn ror16_matches_oracle() {
    rot_test("ror16", 16, VALS16, oracle::ror, Flags::ror);
}

#[test]
fn rol32_matches_oracle() {
    rot_test("rol32", 32, VALS32, oracle::rol, Flags::rol);
}

#[test]
fn ror32_matches_oracle() {
    rot_test("ror32", 32, VALS32, oracle::ror, Flags::ror);
}

// ---- RCL / RCR (rotate through carry) ----

#[test]
fn rcl8_matches_oracle() {
    rc_test("rcl8", 8, VALS8, oracle::rcl, Flags::rcl);
}

#[test]
fn rcr8_matches_oracle() {
    rc_test("rcr8", 8, VALS8, oracle::rcr, Flags::rcr);
}

#[test]
fn rcl16_matches_oracle() {
    rc_test("rcl16", 16, VALS16, oracle::rcl, Flags::rcl);
}

#[test]
fn rcr16_matches_oracle() {
    rc_test("rcr16", 16, VALS16, oracle::rcr, Flags::rcr);
}

#[test]
fn rcl32_matches_oracle() {
    rc_test("rcl32", 32, VALS32, oracle::rcl, Flags::rcl);
}

#[test]
fn rcr32_matches_oracle() {
    rc_test("rcr32", 32, VALS32, oracle::rcr, Flags::rcr);
}
