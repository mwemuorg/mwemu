use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let bytes0 = value0.to_le_bytes();
    let bytes1 = value1.to_le_bytes();

    let mut result_bytes = [0u8; 16];
    result_bytes[0] = bytes0[8];
    result_bytes[1] = bytes1[8];
    result_bytes[2] = bytes0[9];
    result_bytes[3] = bytes1[9];
    result_bytes[4] = bytes0[10];
    result_bytes[5] = bytes1[10];
    result_bytes[6] = bytes0[11];
    result_bytes[7] = bytes1[11];
    result_bytes[8] = bytes0[12];
    result_bytes[9] = bytes1[12];
    result_bytes[10] = bytes0[13];
    result_bytes[11] = bytes1[13];
    result_bytes[12] = bytes0[14];
    result_bytes[13] = bytes1[14];
    result_bytes[14] = bytes0[15];
    result_bytes[15] = bytes1[15];

    let result = u128::from_le_bytes(result_bytes);
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
