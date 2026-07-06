use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sigextend = emu.regs().get_ax() as u16 as i16 as i32 as u32;
    emu.regs_mut().set_ax((sigextend & 0x0000ffff) as u64);
    emu.regs_mut()
        .set_dx(((sigextend & 0xffff0000) >> 16) as u64);
    true
}
