use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // RET {Xn} — default is X30 (LR)
    let raw = if matches!(ins.operands[0], Operand::Nothing) {
        emu.regs_aarch64().x[30]
    } else {
        read_reg(emu, &ins.operands[0])
    };
    let target = match ins.opcode {
        Opcode::RETAA | Opcode::RETAB => pac_strip(raw),
        _ => raw,
    };
    emu.regs_aarch64_mut().pc = target;
    emu.force_reload = true;
    true
}
