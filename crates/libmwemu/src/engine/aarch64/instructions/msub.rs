use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let rn = read_operand_value(emu, &ins.operands[1]);
    let rm = read_operand_value(emu, &ins.operands[2]);
    let ra = read_operand_value(emu, &ins.operands[3]);
    let result = ra.wrapping_sub(rn.wrapping_mul(rm));
    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
