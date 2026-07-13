use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VMOVLHPS dest, src1, src2: dest[63:0]=src1[63:0], dest[127:64]=src2[63:0].
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
        (s1 & 0xffff_ffff_ffff_ffff) | ((s2 & 0xffff_ffff_ffff_ffff) << 64),
    );
    true
}
