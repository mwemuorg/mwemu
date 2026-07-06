use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let rn = read_operand_value(emu, &ins.operands[1]);
    let rm = read_operand_value(emu, &ins.operands[2]);
    let lsb = read_operand_value(emu, &ins.operands[3]);
    let result = if is64 {
        if lsb == 0 {
            rm
        } else {
            (rm >> lsb) | (rn << (64 - lsb))
        }
    } else {
        let lsb = lsb & 31;
        if lsb == 0 {
            rm & 0xffffffff
        } else {
            (((rm as u32) >> lsb) | ((rn as u32) << (32 - lsb))) as u64
        }
    };
    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
