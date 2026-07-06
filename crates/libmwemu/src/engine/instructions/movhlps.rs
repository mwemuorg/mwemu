use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// MOVHLPS xmm1, xmm2: DEST[63:0] = SRC[127:64]; DEST[127:64] unchanged.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let source = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);

    let low_qword = (source >> 64) & 0xFFFFFFFFFFFFFFFF;
    let high_qword = dest & (0xFFFFFFFFFFFFFFFFu128 << 64);
    let result = low_qword | high_qword;

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
