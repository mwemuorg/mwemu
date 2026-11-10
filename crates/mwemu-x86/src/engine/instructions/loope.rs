use crate::color;
use crate::emu::Emu;
use crate::engine::logic;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Yellow"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    debug_assert!(ins.op_count() == 1);

    let addr = match emu.get_jump_value(ins, 0) {
        Some(v) => v,
        None => return false,
    };

    let count = logic::loop_decrement(emu, logic::loop_counter_bits(ins));
    if count != 0 && emu.flag_zf() {
        return logic::loop_jump(emu, addr);
    }
    true
}
