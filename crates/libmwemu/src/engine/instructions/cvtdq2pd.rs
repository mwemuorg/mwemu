use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CVTDQ2PD: low 2 packed signed int32 -> 2 packed f64.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..2 {
        let d = ((src >> (i * 32)) & 0xffffffff) as u32 as i32;
        result |= ((d as f64).to_bits() as u128) << (i * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
