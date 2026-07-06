use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let mut poped: u64;

    // only 32bits instruction
    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_edi(poped);
    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_esi(poped);
    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_ebp(poped);

    let esp = emu.regs().get_esp() + 4;
    emu.regs_mut().set_esp(esp); // skip esp

    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_ebx(poped);
    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_edx(poped);
    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_ecx(poped);
    poped = emu.stack_pop32(false).unwrap_or(0) as u64;
    emu.regs_mut().set_eax(poped);
    true
}
