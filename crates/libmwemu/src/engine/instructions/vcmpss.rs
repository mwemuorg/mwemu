use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let s1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let s2 = try_operand!(emu.get_operand_xmm_value_128(ins, 2, true));
    let imm = try_operand!(emu.get_operand_value(ins, 3, true)) as u8;
    let low: u128 = if avx::cmp_pred_f32(f32::from_bits(s1 as u32), f32::from_bits(s2 as u32), imm)
    {
        0xffffffff
    } else {
        0
    };
    emu.set_operand_xmm_value_128(ins, 0, (s1 & !0xffff_ffffu128) | low)
}
