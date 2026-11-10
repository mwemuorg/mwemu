use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let src = read_operand_value(emu, &ins.operands[1]);
    let immr = read_operand_value(emu, &ins.operands[2]) as u32;
    let imms = read_operand_value(emu, &ins.operands[3]) as u32;
    let bits = if is64 { 64u32 } else { 32 };

    let result = if imms >= immr {
        let width = imms - immr + 1;
        (src >> immr) & ((1u64 << width) - 1)
    } else {
        let width = imms + 1;
        let field = src & ((1u64 << width) - 1);
        let pos = bits - immr;
        field << pos
    };

    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
