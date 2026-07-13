use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMOVSXWD: sign-extend the low 4 16-bit lanes of the source into 32-bit lanes.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..4 {
        let raw = (src >> (i * 16)) & ((1u128 << 16) - 1);
        let sign = 1u128 << (16 - 1);
        let ext = if raw & sign != 0 {
            raw | !((1u128 << 16) - 1)
        } else {
            raw
        };
        result |= (ext & ((1u128 << 32) - 1)) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
