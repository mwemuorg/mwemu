use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = a ^ b;
    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
