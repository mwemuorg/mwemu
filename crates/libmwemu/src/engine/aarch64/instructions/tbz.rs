use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction, is_zero: bool) -> bool {
    let val = read_reg(emu, &ins.operands[0]);
    let bit = read_imm(&ins.operands[1]);
    let bit_val = (val >> bit) & 1;
    let taken = if is_zero { bit_val == 0 } else { bit_val != 0 };
    if taken {
        if let Operand::PCOffset(offset) = ins.operands[2] {
            let pc = emu.regs_aarch64().pc;
            emu.regs_aarch64_mut().pc = pc.wrapping_add(offset as u64);
            emu.force_reload = true;
        }
    }
    true
}
