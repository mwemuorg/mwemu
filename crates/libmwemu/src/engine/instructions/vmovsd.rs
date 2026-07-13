use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VMOVSD: 3-op reg form merges low lane with src1 upper; 2-op mem form zero-extends.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if ins.op_count() == 3 {
        let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
        let s2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
        emu.set_operand_xmm_value_128(
            ins,
            0,
            (s1 & !(0xffff_ffff_ffff_ffff as u128)) | (s2 & 0xffff_ffff_ffff_ffff),
        );
    } else {
        let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
        emu.set_operand_xmm_value_128(ins, 0, s & 0xffff_ffff_ffff_ffff);
    }
    true
}
