use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    // only 32bits instruction
    let tmp_esp = emu.regs().get_esp() as u32;
    if !emu.stack_push32(emu.regs().get_eax() as u32) {
        return false;
    }
    if !emu.stack_push32(emu.regs().get_ecx() as u32) {
        return false;
    }
    if !emu.stack_push32(emu.regs().get_edx() as u32) {
        return false;
    }
    if !emu.stack_push32(emu.regs().get_ebx() as u32) {
        return false;
    }
    if !emu.stack_push32(tmp_esp) {
        return false;
    }
    if !emu.stack_push32(emu.regs().get_ebp() as u32) {
        return false;
    }
    if !emu.stack_push32(emu.regs().get_esi() as u32) {
        return false;
    }
    if !emu.stack_push32(emu.regs().get_edi() as u32) {
        return false;
    }
    true
}
