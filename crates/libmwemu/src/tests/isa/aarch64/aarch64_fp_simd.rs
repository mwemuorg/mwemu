//! Scalar floating-point and SIMD element moves emitted by clang/gcc for
//! ordinary `double`/`float` code. Encodings come from `clang -target arm64`.
use super::{data_map, emu_with, step_ok};
use crate::emu::Emu;

#[test]
fn structure_lane_transfer_preserves_other_lanes_and_updates_address() {
    let mut emu = emu_with(&[
        0x0d40_043f, // ld1 {v31.b}[1], [x1]
        0x0d9f_045f, // st1 {v31.b}[1], [x2], #1
    ]);
    let data = data_map(&mut emu);
    emu.regs_aarch64_mut().x[1] = data;
    emu.regs_aarch64_mut().x[2] = data + 1;
    emu.regs_aarch64_mut().v[31] = u128::MAX;
    emu.maps.write_byte(data, 0x42);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[31], !0xff00_u128 | 0x4200);
    step_ok(&mut emu);
    assert_eq!(emu.maps.read_byte(data + 1), Some(0x42));
    assert_eq!(emu.regs_aarch64().x[2], data + 2);
}

#[test]
fn vector_shifts_zip_and_narrow_preserve_lane_order() {
    let mut emu = emu_with(&[
        0x7f60_07d7, // ushr d23, d30, #32
        0x2e21_3bff, // shll v31.8h, v31.8b, #8
        0x4e51_7bf6, // zip2 v22.8h, v31.8h, v17.8h
        0x0ea1_2b7d, // xtn v29.2s, v27.2d
    ]);
    let regs = emu.regs_aarch64_mut();
    regs.v[30] = u128::MAX;
    regs.v[31] = 0x0807_0605_0403_0201;
    regs.v[27] = 0xaaaa_aaaa_1122_3344_bbbb_bbbb_5566_7788;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[23], u32::MAX as u128);
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[31],
        0x0800_0700_0600_0500_0400_0300_0200_0100
    );
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[22],
        0x0000_0800_0000_0700_0000_0600_0000_0500
    );
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[29], 0x1122_3344_5566_7788);
}

#[test]
fn shift_right_narrow_writes_the_selected_half() {
    let mut emu = emu_with(&[
        0x0f0c_8445, // shrn v5.8b, v2.8h, #4
        0x4f0c_84a5, // shrn2 v5.16b, v5.8h, #4
        0x0f10_8440, // shrn v0.4h, v2.4s, #16
        0x0f20_8440, // shrn v0.2s, v2.2d, #32
    ]);
    emu.regs_aarch64_mut().v[2] = 0xf012_1023_2034_3045_4056_5067_6078_7089;
    emu.regs_aarch64_mut().v[5] = u128::MAX;
    step_ok(&mut emu);
    let low = 0x0102_0304_0506_0708;
    assert_eq!(emu.regs_aarch64().v[5], low);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[5], (0x10305070_u128 << 64) | low);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[0], 0xf012_2034_4056_6078);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[0], 0xf012_1023_4056_5067);
}

#[test]
fn unsigned_comparisons_and_pairwise_max_respect_lane_order() {
    let mut emu = emu_with(&[
        0x6e21_3c62, // cmhs v2.16b, v3.16b, v1.16b
        0x2e21_3464, // cmhi v4.8b, v3.8b, v1.8b
        0x2e21_a463, // umaxp v3.8b, v3.8b, v1.8b
    ]);
    emu.regs_aarch64_mut().v[3] = 0xff01_017f_2277_8801;
    emu.regs_aarch64_mut().v[1] = 0xff00_0280_2222_0188;
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[2],
        0xffff_ffff_ffff_ffff_ffff_0000_ffff_ff00
    );
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[4], 0x00ff_0000_00ff_ff00);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[3], 0xff80_2288_ff7f_7788);
}

#[test]
fn add_high_narrow_discards_each_lanes_carry() {
    let mut emu = emu_with(&[
        0x0e21_4022, // addhn v2.8b, v1.8h, v1.8h
        0x4e61_4000, // addhn2 v0.8h, v0.4s, v1.4s
    ]);
    emu.regs_aarch64_mut().v[1] = 0xff80_0080_ffff_7fff_8080_007f_00ff_0000;
    emu.regs_aarch64_mut().v[0] = 0x0001_0001_ffff_ffff_0001_0000_8001_ffff;
    emu.regs_aarch64_mut().v[2] = u128::MAX;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[2], 0xff01_ffff_0100_0100);
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[0],
        0xff81_ffff_8081_8100_0001_0000_8001_ffff
    );
}

#[test]
fn ext_extracts_bytes_across_two_vectors() {
    let mut emu = emu_with(&[
        0x6e01_4000, // ext v0.16b, v0.16b, v1.16b, #8
        0x2e00_1822, // ext v2.8b, v1.8b, v0.8b, #3
        0x2e00_0022, // ext v2.8b, v1.8b, v0.8b, #0
    ]);
    emu.regs_aarch64_mut().v[0] = 0x0f0e_0d0c_0b0a_0908_0706_0504_0302_0100;
    emu.regs_aarch64_mut().v[1] = 0x1f1e_1d1c_1b1a_1918_1716_1514_1312_1110;
    emu.regs_aarch64_mut().v[2] = u128::MAX;
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[0],
        0x1716_1514_1312_1110_0f0e_0d0c_0b0a_0908
    );
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[2], 0x0a09_0817_1615_1413);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[2], 0x1716_1514_1312_1110);
}

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

#[test]
fn modified_immediates_fill_and_update_every_lane() {
    let mut emu = emu_with(&[
        0x0f00_0420, // movi v0.2s, #1
        0x6f00_c641, // mvni v1.4s, #0x12, msl #8
        0x0f01_3680, // orr v0.2s, #0x34, lsl #8
        0x2f00_3600, // bic v0.2s, #0x10, lsl #8
        0x6f05_e541, // movi v1.2d, #0xff00ff00ff00ff00
    ]);
    emu.regs_aarch64_mut().v[0] = u128::MAX;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[0], 0x0000_0001_0000_0001);
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[1],
        0xffff_ed00_ffff_ed00_ffff_ed00_ffff_ed00
    );
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[0], 0x0000_3401_0000_3401);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[0], 0x0000_2401_0000_2401);
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[1],
        0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00
    );
}

#[test]
fn byte_lane_arithmetic_compares_and_selects_without_carry_between_lanes() {
    let mut emu = emu_with(&[
        0x4e3e_87fb, // add v27.16b, v31.16b, v30.16b
        0x4e20_9bda, // cmeq v26.16b, v30.16b, #0
        0x6e7b_1fda, // bsl v26.16b, v30.16b, v27.16b
    ]);
    emu.regs_aarch64_mut().v[30] = 0x0a00_ff01;
    emu.regs_aarch64_mut().v[31] = u128::from_le_bytes([1; 16]);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[27] as u32, 0x0b01_0002);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[26] as u32, 0x00ff_0000);
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().v[26], 0x0b00_0002);
}

#[test]
fn widening_shift_extends_the_selected_half_before_shifting() {
    let mut emu = emu_with(&[
        0x4f21_a443, // sshll2 v3.2d, v2.4s, #1
        0x2f20_a441, // ushll v1.2d, v2.2s, #0
    ]);
    emu.regs_aarch64_mut().v[2] = 0x8000_0001_0000_0002_ffff_ffff_0000_0001;
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[3],
        0xffff_ffff_0000_0002_0000_0000_0000_0004
    );
    step_ok(&mut emu);
    assert_eq!(
        emu.regs_aarch64().v[1],
        0x0000_0000_ffff_ffff_0000_0000_0000_0001
    );
}

#[test]
fn structure_transfer_interleaves_register_lanes_in_memory() {
    let mut emu = emu_with(&[
        0x0c9f_8400, // st2 {v0.4h, v1.4h}, [x0], #16
        0x0c40_8422, // ld2 {v2.4h, v3.4h}, [x1]
    ]);
    let addr = data_map(&mut emu);
    let regs = emu.regs_aarch64_mut();
    regs.x[0] = addr;
    regs.x[1] = addr;
    regs.v[0] = 0x0007_0005_0003_0001;
    regs.v[1] = 0x0008_0006_0004_0002;
    regs.v[2] = u128::MAX;
    regs.v[3] = u128::MAX;
    step_ok(&mut emu);
    assert_eq!(emu.regs_aarch64().x[0], addr + 16);
    for lane in 0..8 {
        assert_eq!(emu.maps.read_word(addr + lane * 2), Some(lane as u16 + 1));
    }
    step_ok(&mut emu);
    let regs = emu.regs_aarch64();
    assert_eq!(regs.v[2], regs.v[0]);
    assert_eq!(regs.v[3], regs.v[1]);
}
