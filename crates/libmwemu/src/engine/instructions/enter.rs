use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let allocSZ = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let nestingLvl = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let frameTmp = if emu.cfg.is_x64() {
        emu.stack_push64(emu.regs().rbp);
        emu.regs().rsp
    } else {
        emu.stack_push32(emu.regs().get_ebp() as u32);
        emu.regs().get_esp()
    };

    if nestingLvl > 1 {
        for i in 1..nestingLvl {
            if emu.cfg.is_x64() {
                emu.regs_mut().rbp -= 8;
                emu.stack_push64(emu.regs().rbp);
            } else {
                let ebp = emu.regs().get_ebp() - 4;
                emu.regs_mut().set_ebp(ebp);
                emu.stack_push32(emu.regs().get_ebp() as u32);
            }
        }
    } else if emu.cfg.is_x64() {
        emu.stack_push64(frameTmp);
    } else {
        emu.stack_push32(frameTmp as u32);
    }

    if emu.cfg.is_x64() {
        emu.regs_mut().rbp = frameTmp;
        emu.regs_mut().rsp -= allocSZ;
    } else {
        emu.regs_mut().set_ebp(frameTmp);
        let esp = emu.regs().get_esp() - allocSZ;
        emu.regs_mut().set_esp(esp);
    }
    true
}
