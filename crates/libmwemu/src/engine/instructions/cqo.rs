use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sigextend: u128 = emu.regs().rax as i64 as i128 as u128;
    emu.regs_mut().rdx = ((sigextend & 0xffffffff_ffffffff_00000000_00000000) >> 64) as u64;
    true
}
