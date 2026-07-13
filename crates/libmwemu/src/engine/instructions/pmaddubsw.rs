use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMADDUBSW: multiply unsigned bytes of dest by signed bytes of src, add the two
// adjacent products, and saturate each sum to a signed 16-bit word.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for j in 0..8 {
        let base = 16 * j;
        let du0 = ((dest >> base) & 0xff) as u8 as i32;
        let du1 = ((dest >> (base + 8)) & 0xff) as u8 as i32;
        let ss0 = ((src >> base) & 0xff) as u8 as i8 as i32;
        let ss1 = ((src >> (base + 8)) & 0xff) as u8 as i8 as i32;
        let sum = du0 * ss0 + du1 * ss1;
        let sat = sum.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        result |= ((sat as u16) as u128) << base;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
