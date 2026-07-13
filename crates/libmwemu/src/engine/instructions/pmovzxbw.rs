use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMOVZXBW: zero-extend the low 8 8-bit lanes of the source into 16-bit lanes.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..8 {
        let raw = (src >> (i * 8)) & ((1u128 << 8) - 1);
        let ext = raw;
        result |= (ext & ((1u128 << 16) - 1)) << (i * 16);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
