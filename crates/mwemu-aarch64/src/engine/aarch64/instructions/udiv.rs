use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = if b == 0 {
        0
    } else if is64 {
        a / b
    } else {
        ((a as u32) / (b as u32)) as u64
    };
    write_reg(emu, &ins.operands[0], result);
    true
}
