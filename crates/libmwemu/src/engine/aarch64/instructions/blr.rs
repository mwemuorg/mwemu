use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let pc = emu.regs_aarch64().pc;
    let raw = read_reg(emu, &ins.operands[0]);
    let target = match ins.opcode {
        Opcode::BLRAA | Opcode::BLRAAZ | Opcode::BLRAB | Opcode::BLRABZ => pac_strip(raw),
        _ => raw,
    };
    emu.regs_aarch64_mut().x[30] = pc + 4;
    emu.set_pc_aarch64(target);
    emu.force_reload = true;
    true
}
