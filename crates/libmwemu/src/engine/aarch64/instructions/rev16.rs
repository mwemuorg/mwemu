use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_operand_value(emu, &ins.operands[1]);
    // Swap bytes within each 16-bit halfword
    let mut result = 0u64;
    for i in 0..4 {
        let hw = (val >> (i * 16)) & 0xFFFF;
        let swapped = ((hw & 0xFF) << 8) | ((hw >> 8) & 0xFF);
        result |= swapped << (i * 16);
    }
    write_reg(emu, &ins.operands[0], result);
    true
}
