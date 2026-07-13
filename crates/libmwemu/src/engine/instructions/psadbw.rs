use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// PSADBW: sum of absolute byte differences per 64-bit half into its low word.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for half in 0..2u32 {
        let mut sum = 0u32;
        for k in 0..8u32 {
            let bi = half * 8 + k;
            let d = ((dest >> (bi * 8)) & 0xff) as i32;
            let s = ((src >> (bi * 8)) & 0xff) as i32;
            sum += (d - s).unsigned_abs();
        }
        result |= (sum as u128) << (half * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
