use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let result = if b == 0 {
        0 // ARM: division by zero returns 0
    } else if is64 {
        ((a as i64).wrapping_div(b as i64)) as u64
    } else {
        ((a as i32).wrapping_div(b as i32)) as u32 as u64
    };
    write_reg(emu, &ins.operands[0], result);
    true
}
