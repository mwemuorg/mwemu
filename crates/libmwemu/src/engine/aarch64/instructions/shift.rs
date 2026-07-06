use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction, op: ShiftOp) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let a = read_operand_value(emu, &ins.operands[1]);
    let b = read_operand_value(emu, &ins.operands[2]);
    let shift = if is64 { b & 63 } else { b & 31 };
    let result = match op {
        ShiftOp::Lsl => a << shift,
        ShiftOp::Lsr => {
            if is64 {
                a >> shift
            } else {
                ((a as u32) >> shift) as u64
            }
        }
        ShiftOp::Asr => {
            if is64 {
                ((a as i64) >> shift) as u64
            } else {
                ((a as i32) >> (shift as u32)) as u32 as u64
            }
        }
        ShiftOp::Ror => {
            if is64 {
                a.rotate_right(shift as u32)
            } else {
                ((a as u32).rotate_right(shift as u32)) as u64
            }
        }
    };
    let result = if is64 { result } else { result & 0xffffffff };
    write_reg(emu, &ins.operands[0], result);
    true
}
