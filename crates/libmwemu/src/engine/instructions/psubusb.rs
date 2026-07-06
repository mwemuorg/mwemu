use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if emu.get_operand_sz(ins, 0) != 128 || emu.get_operand_sz(ins, 1) != 128 {
        log::trace!("unimplemented");
        return false;
    }

    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..16 {
        let byte0 = ((value0 >> (i * 8)) & 0xFF) as u8;
        let byte1 = ((value1 >> (i * 8)) & 0xFF) as u8;
        let res_byte = byte0.saturating_sub(byte1);

        result |= (res_byte as u128) << (i * 8);
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
