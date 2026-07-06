use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    if let Operand::PCOffset(offset) = ins.operands[0] {
        let pc = emu.regs_aarch64().pc;
        // LR = address of next instruction
        emu.regs_aarch64_mut().x[30] = pc + 4;
        let target = pc.wrapping_add(offset as u64);
        emu.set_pc_aarch64(target);
        emu.force_reload = true;
        true
    } else {
        false
    }
}
