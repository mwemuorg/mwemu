use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let sz = emu.get_operand_sz(ins, 0);
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let imm = try_operand!(emu.get_operand_value(ins, 2, true));
    let n = (imm as u32).wrapping_mul(8); // shift amount in bits

    // Treat dest:src as one double-width value, with dest in the high half.
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

    emu.set_operand_xmm_value_128(ins, 0, result)
}
