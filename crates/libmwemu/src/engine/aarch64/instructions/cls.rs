use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// CLS Rd, Rn: count leading bits equal to the sign bit, excluding it.
pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_reg(emu, &ins.operands[1]);
    let result = if operand_is_64(&ins.operands[0]) {
        let v = val as i64;
        (v ^ (v >> 1)).leading_zeros() as u64 - 1
    } else {
        let v = val as u32 as i32;
        (v ^ (v >> 1)).leading_zeros() as u64 - 1
    };
    write_reg(emu, &ins.operands[0], result);
    true
}
