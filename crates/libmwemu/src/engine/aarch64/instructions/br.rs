use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let target = read_reg(emu, &ins.operands[0]);
    emu.set_pc_aarch64(target);
    emu.force_reload = true;
    true
}
