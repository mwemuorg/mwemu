use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let val = read_operand_value(emu, &ins.operands[1]);
    let result = if is64 { !val } else { (!val) & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
