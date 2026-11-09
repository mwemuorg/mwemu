//! Conversions between integer and floating-point registers, and rounding.
use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

#[derive(Clone, Copy)]
pub enum Rounding {
    /// to nearest, ties to even (N, and X/I under the default FPCR)
    Nearest,
    /// toward +inf (P)
    Up,
    /// toward -inf (M)
    Down,
    /// to nearest, ties away from zero (A)
    Away,
    /// toward zero (Z)
    Zero,
}

pub fn round(v: f64, mode: Rounding) -> f64 {
    match mode {
        Rounding::Nearest => v.round_ties_even(),
        Rounding::Up => v.ceil(),
        Rounding::Down => v.floor(),
        Rounding::Away => v.round(),
        Rounding::Zero => v.trunc(),
    }
}

/// Optional fixed-point fraction bits (third operand), as a scale of 2^fbits.
fn fixed_point_scale(ins: &Instruction) -> f64 {
    match ins.operands[2] {
        Operand::Immediate(fbits) => (fbits as f64).exp2(),
        _ => 1.0,
    }
}

/// SCVTF/UCVTF Fd, Rn{, #fbits}
pub fn int_to_fp(emu: &mut Emu, ins: &Instruction, signed: bool) -> bool {
    let is64 = operand_is_64(&ins.operands[1]);
    let raw = read_reg(emu, &ins.operands[1]);
    let v = match (signed, is64) {
        (true, true) => raw as i64 as f64,
        (true, false) => raw as u32 as i32 as f64,
        (false, _) => raw as f64,
    };
    if !matches!(ins.operands[0], Operand::SIMDRegister(..)) {
        log::warn!("int_to_fp: vector form not supported: {}", ins);
        return false;
    }
    write_fp(emu, &ins.operands[0], v / fixed_point_scale(ins))
}

/// FCVT{N,P,M,A,Z}{S,U} Rd, Fn{, #fbits}. Out-of-range saturates, NaN -> 0.
pub fn fp_to_int(emu: &mut Emu, ins: &Instruction, signed: bool, mode: Rounding) -> bool {
    let Some(v) = read_fp(emu, &ins.operands[1]) else {
        log::warn!("fp_to_int: unsupported operands for {}", ins);
        return false;
    };
    let v = round(v * fixed_point_scale(ins), mode);
    let result = match (signed, operand_is_64(&ins.operands[0])) {
        (true, true) => v as i64 as u64,
        (true, false) => v as i32 as u32 as u64,
        (false, true) => v as u64,
        (false, false) => v as u32 as u64,
    };
    write_reg(emu, &ins.operands[0], result);
    true
}

/// FRINT{N,P,M,A,Z,X,I} Fd, Fn
pub fn frint(emu: &mut Emu, ins: &Instruction, mode: Rounding) -> bool {
    let Some(v) = read_fp(emu, &ins.operands[1]) else {
        log::warn!("frint: unsupported operands for {}", ins);
        return false;
    };
    write_fp(emu, &ins.operands[0], round(v, mode))
}
