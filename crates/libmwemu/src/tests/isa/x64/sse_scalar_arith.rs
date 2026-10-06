//! Scalar SSE arithmetic (`addsd/subsd/mulsd` and the `ss` forms) must operate
//! on the IEEE value of the low lane and leave the upper lanes alone. These
//! used to multiply/add the raw bit patterns as integers, which is wrong for
//! every input and, in debug builds, overflowed on the first real double.

use crate::tests::helpers;
use crate::*;

const HI: u128 = 0x1122_3344_5566_7788 << 64; // upper lane sentinel for xmm0
const SRC_HI: u128 = 0xdead_beef_cafe_babe << 64; // upper lane of xmm1, must not leak

/// Execute one `op xmm0, xmm1` with the given low lanes, return xmm0.
fn scalar_op(opcode: &[u8], xmm0_low: u128, xmm1_low: u128) -> u128 {
    helpers::setup();
    let mut emu = emu64();
    emu.os = crate::arch::OperatingSystem::Linux;
    emu.load_code_bytes(opcode);
    emu.regs_mut().set_xmm_by_name("xmm0", HI | xmm0_low);
    emu.regs_mut().set_xmm_by_name("xmm1", SRC_HI | xmm1_low);
    assert!(emu.step(), "instruction {:x?} should emulate", opcode);
    emu.regs().get_xmm_by_name("xmm0")
}

fn sd(opcode: &[u8], a: f64, b: f64) -> (f64, u128) {
    let r = scalar_op(opcode, a.to_bits() as u128, b.to_bits() as u128);
    (f64::from_bits(r as u64), r >> 64)
}

fn ss(opcode: &[u8], a: f32, b: f32) -> (f32, u128) {
    let r = scalar_op(opcode, a.to_bits() as u128, b.to_bits() as u128);
    assert_eq!((r >> 32) as u32, 0, "bits 32..64 of xmm0 must stay zero");
    (f32::from_bits(r as u32), r >> 64)
}

#[test]
fn mulsd_multiplies_doubles() {
    let (r, hi) = sd(&[0xf2, 0x0f, 0x59, 0xc1], 1.5, -2.0e3);
    assert_eq!(r, -3000.0);
    assert_eq!(hi, HI >> 64, "upper lane preserved");
}

#[test]
fn addsd_adds_doubles() {
    let (r, hi) = sd(&[0xf2, 0x0f, 0x58, 0xc1], 0.1, 0.2);
    assert_eq!(r, 0.1 + 0.2);
    assert_eq!(hi, HI >> 64);
}

#[test]
fn subsd_subtracts_doubles() {
    let (r, hi) = sd(&[0xf2, 0x0f, 0x5c, 0xc1], 1.0, 1e-9);
    assert_eq!(r, 1.0 - 1e-9);
    assert_eq!(hi, HI >> 64);
}

#[test]
fn mulss_multiplies_floats() {
    let (r, hi) = ss(&[0xf3, 0x0f, 0x59, 0xc1], 2.5, 4.0);
    assert_eq!(r, 10.0);
    assert_eq!(hi, HI >> 64);
}

#[test]
fn addss_adds_floats() {
    let (r, hi) = ss(&[0xf3, 0x0f, 0x58, 0xc1], 1.25, -0.25);
    assert_eq!(r, 1.0);
    assert_eq!(hi, HI >> 64);
}

#[test]
fn subss_subtracts_floats() {
    let (r, hi) = ss(&[0xf3, 0x0f, 0x5c, 0xc1], 3.0, 4.5);
    assert_eq!(r, -1.5);
    assert_eq!(hi, HI >> 64);
}
