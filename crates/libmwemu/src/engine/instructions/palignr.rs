use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PALIGNR dest, src, imm8: concatenate dest:src (dest in the high half), shift
// the whole thing right by imm8 bytes, and keep the low operand-width bits.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz = emu.get_operand_sz(ins, 0);
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0);
    let n = (imm as u32).wrapping_mul(8); // shift amount in bits

    let result = if sz == 64 {
        // 64-bit MMX: concatenate the low qwords into a 128-bit value.
        let concat = ((dest & 0xffff_ffff_ffff_ffff) << 64) | (src & 0xffff_ffff_ffff_ffff);
        let shifted = if n >= 128 { 0 } else { concat >> n };
        shifted & 0xffff_ffff_ffff_ffff
    } else if n >= 256 {
        0
    } else if n >= 128 {
        let s = n - 128;
        if s >= 128 { 0 } else { dest >> s }
    } else if n == 0 {
        src
    } else {
        (src >> n) | (dest << (128 - n))
    };

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
