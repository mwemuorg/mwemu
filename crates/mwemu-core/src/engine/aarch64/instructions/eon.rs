use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// EON Rd, Rn, Rm{, shift}: Rd = Rn ^ !Rm
pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    write_reg(emu, &ins.operands[0], a ^ !b);
    true
}
