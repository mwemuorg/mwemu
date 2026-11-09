use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_operand_value(emu, &ins.operands[1]);
    write_reg(emu, &ins.operands[0], val);
    true
}
