use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// SHUFPD xmm1, xmm2/m128, imm8
//   DEST[63:0]   = imm8[0] ? DEST[127:64] : DEST[63:0]
//   DEST[127:64] = imm8[1] ? SRC[127:64]  : SRC[63:0]
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0);

    let low = if imm & 1 != 0 {
        dest >> 64
    } else {
        dest & 0xFFFFFFFFFFFFFFFF
    };
    let high = if imm & 2 != 0 {
        src >> 64
    } else {
        src & 0xFFFFFFFFFFFFFFFF
    };
    let result = (low & 0xFFFFFFFFFFFFFFFF) | ((high & 0xFFFFFFFFFFFFFFFF) << 64);

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
