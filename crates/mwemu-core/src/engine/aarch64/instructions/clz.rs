use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let val = read_operand_value(emu, &ins.operands[1]);
    let result = if is64 {
        val.leading_zeros() as u64
    } else {
        (val as u32).leading_zeros() as u64
    };
    write_reg(emu, &ins.operands[0], result);
    true
}
