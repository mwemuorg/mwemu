use crate::color;
use crate::emu::Emu;
use crate::engine::logic;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    debug_assert!(ins.op_count() == 2);

    let target = match logic::bit_target(emu, ins) {
        Some(t) => t,
        None => return false,
    };
    let cf = target.selected_bit();
    emu.flags_mut().f_cf = cf;
    let mut result = target.value;
    set_bit!(result, target.bit, 0);
    logic::bit_target_store(emu, ins, &target, result)
}
