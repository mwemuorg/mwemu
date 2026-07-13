use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PACKUSDW: pack signed dwords into unsigned words with saturation to [0,65535]
// (dest dwords -> low words, src dwords -> high words).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let sat = |v: i32| -> u128 { v.clamp(0, 0xffff) as u128 };
    let mut result = 0u128;
    for j in 0..4 {
        let d = ((dest >> (j * 32)) & 0xffffffff) as u32 as i32;
        result |= sat(d) << (16 * j);
        let s = ((src >> (j * 32)) & 0xffffffff) as u32 as i32;
        result |= sat(s) << (16 * (j + 4));
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
