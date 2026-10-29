//! Packed SSE float arithmetic (`addps/subps/mulps` and the `pd` forms) must
//! operate per IEEE lane: the old handlers added/subtracted/multiplied the raw
//! bit patterns as integers. The NaN tests pin the x86 rules (SDM Vol. 1,
//! Table 4-7) so results no longer depend on the host FPU: an invalid
//! operation yields the x86 "real indefinite" QNaN (sign bit set), a NaN input
//! propagates (quieted), and with two NaN inputs the first operand wins.

use crate::tests::helpers;
use crate::*;

fn op(opcode: &[u8], xmm0: u128, xmm1: u128) -> u128 {
    helpers::setup();
    let mut emu = emu64();
    emu.os = crate::arch::OperatingSystem::Linux;
    emu.load_code_bytes(opcode);
    emu.regs_mut().set_xmm_by_name("xmm0", xmm0);
    emu.regs_mut().set_xmm_by_name("xmm1", xmm1);
    assert!(emu.step(), "instruction {:x?} should emulate", opcode);
    emu.regs().get_xmm_by_name("xmm0")
}

fn ps(lanes: [f32; 4]) -> u128 {
    lanes.iter().enumerate().fold(0u128, |acc, (i, l)| {
        acc | ((l.to_bits() as u128) << (32 * i))
    })
}

fn pd(lanes: [f64; 2]) -> u128 {
    lanes.iter().enumerate().fold(0u128, |acc, (i, l)| {
        acc | ((l.to_bits() as u128) << (64 * i))
    })
}

const ADDPS: &[u8] = &[0x0f, 0x58, 0xc1];
const SUBPS: &[u8] = &[0x0f, 0x5c, 0xc1];
const MULPS: &[u8] = &[0x0f, 0x59, 0xc1];
const ADDPD: &[u8] = &[0x66, 0x0f, 0x58, 0xc1];
const SUBPD: &[u8] = &[0x66, 0x0f, 0x5c, 0xc1];
const MULPD: &[u8] = &[0x66, 0x0f, 0x59, 0xc1];
const ADDSS: &[u8] = &[0xf3, 0x0f, 0x58, 0xc1];
const SUBSS: &[u8] = &[0xf3, 0x0f, 0x5c, 0xc1];
const DIVSS: &[u8] = &[0xf3, 0x0f, 0x5e, 0xc1];
const SQRTSS: &[u8] = &[0xf3, 0x0f, 0x51, 0xc1];
const ADDSD: &[u8] = &[0xf2, 0x0f, 0x58, 0xc1];
const VADDSS: &[u8] = &[0xc5, 0xf2, 0x58, 0xc2]; // vaddss xmm0, xmm1, xmm2

const F32_INDEFINITE: u32 = 0xffc0_0000;
const F64_INDEFINITE: u64 = 0xfff8_0000_0000_0000;

#[test]
fn addps_adds_float_lanes() {
    let r = op(ADDPS, ps([1.0, 2.0, 3.0, 4.0]), ps([0.5; 4]));
    assert_eq!(r, ps([1.5, 2.5, 3.5, 4.5]));
}

#[test]
fn subps_subtracts_float_lanes() {
    let r = op(SUBPS, ps([1.0, 2.0, 3.0, 4.0]), ps([0.5; 4]));
    assert_eq!(r, ps([0.5, 1.5, 2.5, 3.5]));
}

#[test]
fn mulps_multiplies_float_lanes() {
    let r = op(MULPS, ps([1.5, -2.0, 3.0, 0.25]), ps([2.0; 4]));
    assert_eq!(r, ps([3.0, -4.0, 6.0, 0.5]));
}

#[test]
fn addpd_adds_double_lanes() {
    let r = op(ADDPD, pd([1.5, 2.5]), pd([0.25, 0.25]));
    assert_eq!(r, pd([1.75, 2.75]));
}

#[test]
fn subpd_subtracts_double_lanes() {
    let r = op(SUBPD, pd([1.5, 2.5]), pd([0.25, 0.25]));
    assert_eq!(r, pd([1.25, 2.25]));
}

#[test]
fn mulpd_multiplies_double_lanes() {
    let r = op(MULPD, pd([1.5, -2.0]), pd([2.0, 4.0]));
    assert_eq!(r, pd([3.0, -8.0]));
}

#[test]
fn invalid_operation_yields_x86_indefinite_f32() {
    let inf = f32::INFINITY.to_bits() as u128;
    assert_eq!(op(SUBSS, inf, inf) as u32, F32_INDEFINITE, "inf - inf");
    assert_eq!(op(DIVSS, 0, 0) as u32, F32_INDEFINITE, "0 / 0");
    let neg = (-1.0f32).to_bits() as u128;
    assert_eq!(op(SQRTSS, 0, neg) as u32, F32_INDEFINITE, "sqrt(-1)");
}

#[test]
fn invalid_operation_yields_x86_indefinite_f64() {
    let inf = f64::INFINITY.to_bits() as u128;
    let ninf = f64::NEG_INFINITY.to_bits() as u128;
    assert_eq!(op(ADDSD, inf, ninf) as u64, F64_INDEFINITE, "inf + -inf");
}

#[test]
fn invalid_operation_in_packed_lane_yields_indefinite() {
    let a = ps([f32::INFINITY, 1.0, 2.0, 3.0]);
    let b = ps([f32::NEG_INFINITY, 1.0, 1.0, 1.0]);
    let r = op(ADDPS, a, b);
    assert_eq!(r as u32, F32_INDEFINITE, "lane 0 is invalid");
    assert_eq!(
        r >> 32,
        ps([2.0, 3.0, 4.0, 0.0]),
        "other lanes are normal sums"
    );
}

#[test]
fn vex_invalid_operation_yields_indefinite() {
    helpers::setup();
    let mut emu = emu64();
    emu.os = crate::arch::OperatingSystem::Linux;
    emu.load_code_bytes(VADDSS);
    emu.regs_mut()
        .set_xmm_by_name("xmm1", f32::INFINITY.to_bits() as u128);
    emu.regs_mut()
        .set_xmm_by_name("xmm2", f32::NEG_INFINITY.to_bits() as u128);
    assert!(emu.step());
    assert_eq!(emu.regs().get_xmm_by_name("xmm0") as u32, F32_INDEFINITE);
}

#[test]
fn qnan_input_propagates_with_payload() {
    let qnan = 0x7fc0_0123u128;
    let one = 1.0f32.to_bits() as u128;
    assert_eq!(op(ADDSS, qnan, one) as u32, 0x7fc0_0123, "dest NaN");
    assert_eq!(op(ADDSS, one, qnan) as u32, 0x7fc0_0123, "src NaN");
    let nqnan = 0xffc0_0005u128;
    assert_eq!(
        op(ADDSS, one, nqnan) as u32,
        0xffc0_0005,
        "negative NaN keeps its sign"
    );
}

#[test]
fn two_nan_inputs_first_operand_wins() {
    let a = 0x7fc0_0111u128;
    let b = 0x7fc0_0222u128;
    assert_eq!(op(ADDSS, a, b) as u32, 0x7fc0_0111);
    assert_eq!(op(ADDSS, b, a) as u32, 0x7fc0_0222);
}

#[test]
fn snan_input_is_quieted() {
    let snan = 0x7f80_0001u128;
    let one = 1.0f32.to_bits() as u128;
    assert_eq!(op(ADDSS, one, snan) as u32, 0x7fc0_0001, "src SNaN quieted");
    let qnan = 0x7fc0_0222u128;
    assert_eq!(
        op(ADDSS, snan, qnan) as u32,
        0x7fc0_0001,
        "first operand (SNaN) wins and is quieted"
    );
}
