use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let selector = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v as u16,
        None => return false,
    };

    // In long mode (64-bit), flat segments have limit 0xFFFFFFFF.
    // ntdll uses LSL to check the segment type (e.g. IsThreadAFiber).
    let limit: u64 = if selector == 0 {
        emu.flags_mut().f_zf = false;
        return true;
    } else {
        0xFFFFFFFF
    };

    emu.flags_mut().f_zf = true;
    emu.set_operand_value(ins, 0, limit);
    true
}
