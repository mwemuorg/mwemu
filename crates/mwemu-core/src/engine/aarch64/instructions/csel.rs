use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let cond = match ins.operands[3] {
        Operand::ConditionCode(c) => c,
        _ => return false,
    };
    let result = if emu.regs_aarch64().nzcv.eval_condition(cond) {
        read_operand_value(emu, &ins.operands[1])
    } else {
        read_operand_value(emu, &ins.operands[2])
    };
    write_reg(emu, &ins.operands[0], result);
    true
}
