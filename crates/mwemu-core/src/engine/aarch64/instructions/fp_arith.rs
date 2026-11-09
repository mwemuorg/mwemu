//! Scalar floating-point arithmetic on S and D registers.
use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

#[derive(Clone, Copy)]
pub enum FpBinary {
    Add,
    Sub,
    Mul,
    Div,
    Max,
    Min,
    MaxNm,
    MinNm,
    NMul,
}

#[derive(Clone, Copy)]
pub enum FpUnary {
    Abs,
    Neg,
    Sqrt,
    Mov,
}

#[derive(Clone, Copy)]
pub enum FpFused {
    MAdd,
    MSub,
    NMAdd,
    NMSub,
}

/// ARM FMAX/FMIN propagate NaN; the NM variants prefer the number.
fn propagating(a: f64, b: f64, pick: fn(f64, f64) -> f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        pick(a, b)
    }
}

/// Fd = Fn <op> Fm
pub fn binary(emu: &mut Emu, ins: &Instruction, op: FpBinary) -> bool {
    let (Some(a), Some(b)) = (
        read_fp(emu, &ins.operands[1]),
        read_fp(emu, &ins.operands[2]),
    ) else {
        log::warn!("fp: unsupported operands for {}", ins);
        return false;
    };
    let r = match op {
        FpBinary::Add => a + b,
        FpBinary::Sub => a - b,
        FpBinary::Mul => a * b,
        FpBinary::Div => a / b,
        FpBinary::Max => propagating(a, b, f64::max),
        FpBinary::Min => propagating(a, b, f64::min),
        FpBinary::MaxNm => a.max(b),
        FpBinary::MinNm => a.min(b),
        FpBinary::NMul => -(a * b),
    };
    write_fp(emu, &ins.operands[0], r)
}

/// Fd = <op>(Fn)
pub fn unary(emu: &mut Emu, ins: &Instruction, op: FpUnary) -> bool {
    let Some(a) = read_fp(emu, &ins.operands[1]) else {
        log::warn!("fp: unsupported operands for {}", ins);
        return false;
    };
    let r = match op {
        FpUnary::Abs => a.abs(),
        FpUnary::Neg => -a,
        FpUnary::Sqrt => a.sqrt(),
        FpUnary::Mov => a,
    };
    write_fp(emu, &ins.operands[0], r)
}

/// Fd = +/-Fa +/- Fn*Fm, fused (single rounding).
pub fn fused(emu: &mut Emu, ins: &Instruction, op: FpFused) -> bool {
    let (Some(n), Some(m), Some(a)) = (
        read_fp(emu, &ins.operands[1]),
        read_fp(emu, &ins.operands[2]),
        read_fp(emu, &ins.operands[3]),
    ) else {
        log::warn!("fp: unsupported operands for {}", ins);
        return false;
    };
    let r = match op {
        FpFused::MAdd => n.mul_add(m, a),
        FpFused::MSub => (-n).mul_add(m, a),
        FpFused::NMAdd => (-n).mul_add(m, -a),
        FpFused::NMSub => n.mul_add(m, -a),
    };
    write_fp(emu, &ins.operands[0], r)
}
