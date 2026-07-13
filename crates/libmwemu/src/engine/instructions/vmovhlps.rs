use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VMOVHLPS dest, src1, src2: dest[63:0]=src2[127:64], dest[127:64]=src1[127:64].
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let s2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    emu.set_operand_xmm_value_128(
        ins,
        0,
        ((s2 >> 64) & 0xffff_ffff_ffff_ffff) | (((s1 >> 64) & 0xffff_ffff_ffff_ffff) << 64),
    );
    true
}
