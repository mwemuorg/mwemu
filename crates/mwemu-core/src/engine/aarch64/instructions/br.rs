use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let raw = read_reg(emu, &ins.operands[0]);
    let target = match ins.opcode {
        Opcode::BRAA | Opcode::BRAAZ | Opcode::BRAB | Opcode::BRABZ => pac_strip(raw),
        _ => raw,
    };
    emu.set_pc_aarch64(target);
    emu.force_reload = true;
    true
}
