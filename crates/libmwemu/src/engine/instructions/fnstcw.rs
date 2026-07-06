use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let addr = emu.get_operand_value(ins, 0, false).unwrap_or(0);
    if addr > 0 {
        emu.maps.write_word(addr, emu.fpu().fpu_control_word);
    }
    true
}
