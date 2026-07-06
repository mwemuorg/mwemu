use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    match emu.regs().get_ecx() {
        0 => {
            emu.regs_mut().set_edx(0);
            emu.regs_mut().set_eax(0x1f); //7
        }
        _ => {
            emu.regs_mut().set_edx(0);
            emu.regs_mut().set_eax(7);
        }
    }
    true
}
