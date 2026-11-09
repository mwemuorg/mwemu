//! Floating-point compares and conditional select.
use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

/// NZCV result of an IEEE compare: lt=1000, eq=0110, gt=0010, unordered=0011.
fn compare_flags(a: f64, b: f64) -> u64 {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Less) => 0b1000,
        Some(std::cmp::Ordering::Equal) => 0b0110,
        Some(std::cmp::Ordering::Greater) => 0b0010,
        None => 0b0011,
    }
}

fn set_nzcv(emu: &mut Emu, nzcv: u64) {
    emu.regs_aarch64_mut().nzcv.from_u64(nzcv << 28);
}

/// FCMP/FCMPE Fn, Fm|#0.0
pub fn fcmp(emu: &mut Emu, ins: &Instruction) -> bool {
    let a = read_fp(emu, &ins.operands[0]);
    let b = match ins.operands[1] {
        Operand::ImmediateDouble(v) => Some(v),
        ref op => read_fp(emu, op),
    };
    let (Some(a), Some(b)) = (a, b) else {
        log::warn!("fcmp: unsupported operands for {}", ins);
        return false;
    };
    set_nzcv(emu, compare_flags(a, b));
    true
}

/// FCCMP/FCCMPE Fn, Fm, #nzcv, cond
pub fn fccmp(emu: &mut Emu, ins: &Instruction) -> bool {
    let Operand::ConditionCode(cond) = ins.operands[3] else {
        return false;
    };
    if !emu.regs_aarch64().nzcv.eval_condition(cond) {
        set_nzcv(emu, read_imm(&ins.operands[2]) & 0xf);
        return true;
    }
    fcmp(emu, ins)
}

/// FCSEL Fd, Fn, Fm, cond
pub fn fcsel(emu: &mut Emu, ins: &Instruction) -> bool {
    let Operand::ConditionCode(cond) = ins.operands[3] else {
        return false;
    };
    let src = if emu.regs_aarch64().nzcv.eval_condition(cond) {
        &ins.operands[1]
    } else {
        &ins.operands[2]
    };
    let Some(v) = read_fp(emu, src) else {
        log::warn!("fcsel: unsupported operands for {}", ins);
        return false;
    };
    write_fp(emu, &ins.operands[0], v)
}
