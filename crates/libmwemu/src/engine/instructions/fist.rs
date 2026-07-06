use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value = emu.fpu_mut().get_st(0) as i64;
    let value2 = match emu.get_operand_sz(ins, 0) {
        16 => value as i16 as u16 as u64,
        32 => value as i32 as u32 as u64,
        64 => value as u64,
        _ => return false,
    };
    emu.set_operand_value(ins, 0, value2);
    emu.sync_fpu_ip();
    true
}
