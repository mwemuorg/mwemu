use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let pc = emu.regs_aarch64().pc;
    if let Operand::PCOffset(offset) = ins.operands[1] {
        let result = pc.wrapping_add(offset as u64);
        write_reg(emu, &ins.operands[0], result);
        true
    } else {
        false
    }
}
