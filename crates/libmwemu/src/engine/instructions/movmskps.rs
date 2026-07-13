use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// MOVMSKPS: gather the sign bit of each lane into the low bits of a GPR.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut m = 0u64;
    for i in 0..4 {
        if (s >> (i * 32 + 32 - 1)) & 1 == 1 {
            m |= 1 << i;
        }
    }
    if !emu.set_operand_value(ins, 0, m) {
        return false;
    }
    true
}
