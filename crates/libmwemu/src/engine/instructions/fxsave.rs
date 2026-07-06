use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v,
        None => return false,
    };

    let state = emu.fpu().fxsave();
    state.save(addr, emu);
    emu.sync_fpu_ip();
    true
}
