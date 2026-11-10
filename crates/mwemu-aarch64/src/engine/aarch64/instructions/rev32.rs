use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_operand_value(emu, &ins.operands[1]);
    let lo = (val as u32).swap_bytes() as u64;
    let hi = ((val >> 32) as u32).swap_bytes() as u64;
    let result = (hi << 32) | lo;
    write_reg(emu, &ins.operands[0], result);
    true
}
