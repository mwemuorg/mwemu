use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // RET {Xn} — default is X30 (LR)
    let target = if matches!(ins.operands[0], Operand::Nothing) {
        emu.regs_aarch64().x[30]
    } else {
        read_reg(emu, &ins.operands[0])
    };
    emu.regs_aarch64_mut().pc = target;
    emu.force_reload = true;
    true
}
