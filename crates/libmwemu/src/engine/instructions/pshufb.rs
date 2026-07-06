use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PSHUFB xmm1, xmm2/m128 : per-byte shuffle. For each byte i, if the control
// byte's high bit is set the result byte is 0, otherwise it selects
// dest[control[i] & 0x0F].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let dest = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let ctrl = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let src_bytes = dest.to_le_bytes();
    let ctrl_bytes = ctrl.to_le_bytes();
    let mut result = [0u8; 16];
    for i in 0..16 {
        let c = ctrl_bytes[i];
        if c & 0x80 == 0 {
            result[i] = src_bytes[(c & 0x0F) as usize];
        }
    }

    emu.set_operand_xmm_value_128(ins, 0, u128::from_le_bytes(result));
    true
}
