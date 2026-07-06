use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let cur = read_reg(emu, &ins.operands[0]);
    // MOVK keeps other bits, inserts 16-bit immediate at shifted position
    let (imm, shift) = match ins.operands[1] {
        Operand::ImmShift(v, s) => (v as u64, s as u64),
        Operand::Immediate(v) => (v as u64, 0_u64),
        Operand::Imm16(v) => (v as u64, 0_u64),
        _ => panic!("MOVK unexpected operand: {:?}", ins.operands[1]),
    };
    let mask = 0xFFFF_u64 << shift;
    let result = (cur & !mask) | ((imm & 0xFFFF) << shift);
    write_reg(emu, &ins.operands[0], result);
    true
}
