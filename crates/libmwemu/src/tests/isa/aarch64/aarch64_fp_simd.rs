//! Scalar floating-point and SIMD element moves emitted by clang/gcc for
//! ordinary `double`/`float` code. Encodings come from `clang -target arm64`.
use super::{emu_with, step_ok};
use crate::emu::Emu;

fn set_d(emu: &mut Emu, r: usize, v: f64) {
    emu.regs_aarch64_mut().v[r] = v.to_bits() as u128;
}

fn get_d(emu: &Emu, r: usize) -> f64 {
    f64::from_bits(emu.regs_aarch64().v[r] as u64)
}

fn set_s(emu: &mut Emu, r: usize, v: f32) {
    emu.regs_aarch64_mut().v[r] = v.to_bits() as u128;
}

fn get_s(emu: &Emu, r: usize) -> f32 {
    f32::from_bits(emu.regs_aarch64().v[r] as u32)
}

/// Run a `<op> d0, d1, d2` instruction and return d0.
fn binop_d(word: u32, a: f64, b: f64) -> f64 {
    let mut emu = emu_with(&[word]);
    set_d(&mut emu, 1, a);
    set_d(&mut emu, 2, b);
    step_ok(&mut emu);
    get_d(&emu, 0)
}

/// Run a `<op> d0, d1` instruction and return d0.
fn unop_d(word: u32, a: f64) -> f64 {
    let mut emu = emu_with(&[word]);
    set_d(&mut emu, 1, a);
    step_ok(&mut emu);
    get_d(&emu, 0)
}

/// Run a `<cvt> x0|w0, d1` instruction and return x0.
fn cvt_to_int(word: u32, a: f64) -> u64 {
    let mut emu = emu_with(&[word]);
    emu.regs_aarch64_mut().x[0] = 0xDEAD_BEEF_DEAD_BEEF;
    set_d(&mut emu, 1, a);
    step_ok(&mut emu);
    emu.regs_aarch64().x[0]
}

/// Run `fcmp d0, d1` and return NZCV as a 4-bit value.
fn fcmp_flags(a: f64, b: f64) -> u64 {
    let mut emu = emu_with(&[0x1e612000]); // fcmp d0, d1
    set_d(&mut emu, 0, a);
    set_d(&mut emu, 1, b);
    step_ok(&mut emu);
    emu.regs_aarch64().nzcv.as_u64() >> 28
}

#[test]
fn double_arithmetic() {
    assert_eq!(binop_d(0x1e622820, 1.5, 2.25), 3.75); // fadd
    assert_eq!(binop_d(0x1e620820, 3.0, -2.0), -6.0); // fmul
    assert_eq!(binop_d(0x1e621820, 1.0, 4.0), 0.25); // fdiv
    assert_eq!(binop_d(0x1e621820, 1.0, 0.0), f64::INFINITY); // fdiv by zero
    assert_eq!(binop_d(0x1e628820, 2.0, 3.0), -6.0); // fnmul
    assert_eq!(binop_d(0x1e624820, 1.0, 2.0), 2.0); // fmax
    assert_eq!(binop_d(0x1e625820, 1.0, 2.0), 1.0); // fmin
    assert!(binop_d(0x1e624820, f64::NAN, 2.0).is_nan()); // fmax propagates NaN
    assert_eq!(binop_d(0x1e626820, f64::NAN, 2.0), 2.0); // fmaxnm prefers number
}

#[test]
fn single_arithmetic_zeroes_upper_lanes() {
    let mut emu = emu_with(&[0x1e223820]); // fsub s0, s1, s2
    emu.regs_aarch64_mut().v[0] = u128::MAX;
    set_s(&mut emu, 1, 10.0);
    set_s(&mut emu, 2, 0.5);
    step_ok(&mut emu);
    assert_eq!(get_s(&emu, 0), 9.5);
    assert_eq!(emu.regs_aarch64().v[0] >> 32, 0);
}

#[test]
fn fused_multiply_add_family() {
    for (word, expected) in [(0x1f420c20u32, 7.0), (0x1f428c20, -5.0), (0x1f620c20, -7.0)] {
        // fmadd / fmsub / fnmadd d0, d1, d2, d3 with d1=2, d2=3, d3=1
        let mut emu = emu_with(&[word]);
        set_d(&mut emu, 1, 2.0);
        set_d(&mut emu, 2, 3.0);
        set_d(&mut emu, 3, 1.0);
        step_ok(&mut emu);
        assert_eq!(get_d(&emu, 0), expected, "word 0x{:08x}", word);
    }
}

#[test]
fn unary_ops() {
    assert_eq!(unop_d(0x1e60c020, -3.0), 3.0); // fabs
    assert_eq!(unop_d(0x1e614020, 3.0), -3.0); // fneg
    assert_eq!(unop_d(0x1e61c020, 16.0), 4.0); // fsqrt
    assert_eq!(unop_d(0x1e654020, -1.5), -2.0); // frintm
    assert_eq!(unop_d(0x1e664020, 2.5), 3.0); // frinta
    assert_eq!(unop_d(0x1e644020, 2.5), 2.0); // frintn (ties to even)
    assert_eq!(unop_d(0x1e64c020, 2.1), 3.0); // frintp
    assert_eq!(unop_d(0x1e65c020, -1.7), -1.0); // frintz
}

#[test]
fn precision_conversions() {
    let mut emu = emu_with(&[0x1e22c020]); // fcvt d0, s1
    set_s(&mut emu, 1, 1.5);
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), 1.5);

    let mut emu = emu_with(&[0x1e624020]); // fcvt s0, d1
    set_d(&mut emu, 1, 0.1);
    step_ok(&mut emu);
    assert_eq!(get_s(&emu, 0), 0.1f32);
}

#[test]
fn fcmp_sets_ieee_flags() {
    assert_eq!(fcmp_flags(1.0, 2.0), 0b1000, "less than");
    assert_eq!(fcmp_flags(2.0, 2.0), 0b0110, "equal");
    assert_eq!(fcmp_flags(3.0, 2.0), 0b0010, "greater than");
    assert_eq!(fcmp_flags(f64::NAN, 1.0), 0b0011, "unordered");

    let mut emu = emu_with(&[0x1e602008]); // fcmp d0, #0.0
    set_d(&mut emu, 0, 0.0);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().nzcv.as_u64() >> 28, 0b0110);
}

#[test]
fn fccmp_uses_immediate_when_condition_fails() {
    let mut emu = emu_with(&[0x1e611404, 0x1e611404]); // fccmp d0, d1, #4, ne (x2)
    set_d(&mut emu, 0, 1.0);
    set_d(&mut emu, 1, 2.0);
    emu.regs_aarch64_mut().nzcv.z = true; // NE fails
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().nzcv.as_u64() >> 28, 0b0100);

    emu.regs_aarch64_mut().nzcv.z = false; // NE holds -> real compare
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().nzcv.as_u64() >> 28, 0b1000);
}

#[test]
fn fcsel_selects_by_condition() {
    let mut emu = emu_with(&[0x1e620c20, 0x1e620c20]); // fcsel d0, d1, d2, eq (x2)
    set_d(&mut emu, 1, 1.0);
    set_d(&mut emu, 2, 2.0);
    emu.regs_aarch64_mut().nzcv.z = true;
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), 1.0);
    emu.regs_aarch64_mut().nzcv.z = false;
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), 2.0);
}

#[test]
fn int_to_float() {
    let mut emu = emu_with(&[0x9e620020]); // scvtf d0, x1
    emu.regs_aarch64_mut().x[1] = (-7i64) as u64;
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), -7.0);

    let mut emu = emu_with(&[0x1e220020]); // scvtf s0, w1
    emu.regs_aarch64_mut().x[1] = 0x1_FFFF_FFFF; // w1 = -1
    step_ok(&mut emu);
    assert_eq!(get_s(&emu, 0), -1.0);

    let mut emu = emu_with(&[0x9e630020]); // ucvtf d0, x1
    emu.regs_aarch64_mut().x[1] = u64::MAX;
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), u64::MAX as f64);

    let mut emu = emu_with(&[0x9e42f020]); // scvtf d0, x1, #4 (fixed point)
    emu.regs_aarch64_mut().x[1] = 24;
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), 1.5);
}

#[test]
fn float_to_int_rounding_and_saturation() {
    assert_eq!(cvt_to_int(0x9e780020, -2.9), (-2i64) as u64); // fcvtzs x0, d1
    assert_eq!(cvt_to_int(0x9e780020, 1e30), i64::MAX as u64); // saturates
    assert_eq!(cvt_to_int(0x9e780020, f64::NAN), 0); // NaN -> 0
    assert_eq!(cvt_to_int(0x9e790020, -1.0), 0); // fcvtzu clamps negatives
    assert_eq!(cvt_to_int(0x9e700020, -2.5), (-3i64) as u64); // fcvtms (floor)
    assert_eq!(cvt_to_int(0x1e680020, 2.1), 3); // fcvtps w0 (ceil)
    assert_eq!(cvt_to_int(0x9e640020, -2.5), (-3i64) as u64); // fcvtas (ties away)
    assert_eq!(cvt_to_int(0x9e600020, 2.5), 2); // fcvtns (ties even)
    assert_eq!(cvt_to_int(0x9e58f020, 1.5), 24); // fcvtzs x0, d1, #4

    let mut emu = emu_with(&[0x1e380020]); // fcvtzs w0, s1
    emu.regs_aarch64_mut().x[0] = u64::MAX;
    set_s(&mut emu, 1, -3.7);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xFFFF_FFFD, "w-form zero-extends");
}

#[test]
fn fmov_immediate() {
    let mut emu = emu_with(&[0x1e6e1000, 0x1e309000]); // fmov d0, #1.0 ; fmov s0, #-2.5
    step_ok(&mut emu);
    assert_eq!(get_d(&emu, 0), 1.0);
    step_ok(&mut emu);
    assert_eq!(get_s(&emu, 0), -2.5);
    assert_eq!(emu.regs_aarch64().v[0] >> 32, 0);
}

#[test]
fn umov_extracts_lanes() {
    let mut emu = emu_with(&[0x0e073c20, 0x4e183c20]); // umov w0, v1.b[3] ; mov x0, v1.d[1]
    emu.regs_aarch64_mut().v[1] = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_FF00;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0xDD);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], 0x1122_3344_5566_7788);
}

#[test]
fn ins_replaces_one_lane() {
    let mut emu = emu_with(&[0x4e181c20, 0x6e0c0420]); // mov v0.d[1], x1 ; mov v0.s[1], v1.s[0]
    emu.regs_aarch64_mut().v[0] = 0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_CCCC_CCCC;
    emu.regs_aarch64_mut().x[1] = 0x1234;
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[0],
        0x0000_0000_0000_1234_BBBB_BBBB_CCCC_CCCC
    );

    emu.regs_aarch64_mut().v[1] = 0xFFFF_FFFF_5555_5555;
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[0],
        0x0000_0000_0000_1234_5555_5555_CCCC_CCCC
    );
}

#[test]
fn not_inverts_vector() {
    let mut emu = emu_with(&[0x6e205820]); // mvn v0.16b, v1.16b
    emu.regs_aarch64_mut().v[1] = 0x00FF_00FF;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[0], !0x00FF_00FFu128);
}
