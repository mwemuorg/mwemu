use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CVTPD2PS: 2 packed f64 -> low 2 packed f32 (high 64 bits zeroed).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..2 {
        let f = f64::from_bits(((src >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
        result |= ((f as f32).to_bits() as u128) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
