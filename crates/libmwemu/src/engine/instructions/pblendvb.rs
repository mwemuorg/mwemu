use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// PBLENDVB: per-lane select from src when the XMM0 mask lane's high bit is set.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mask = emu.regs().xmm0;
    let mut result = 0u128;
    for i in 0..16 {
        let shift = i * 8;
        let sel = (mask >> (shift + 8 - 1)) & 1 == 1;
        let lane = if sel {
            (src >> shift) & 0xff
        } else {
            (dest >> shift) & 0xff
        };
        result |= lane << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
