use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mask = emu.regs().xmm0;
    let mut result = 0u128;
    for i in 0..2 {
        let shift = i * 64;
        let sel = (mask >> (shift + 64 - 1)) & 1 == 1;
        let lane = if sel {
            (src >> shift) & 0xffff_ffff_ffff_ffff
        } else {
            (dest >> shift) & 0xffff_ffff_ffff_ffff
        };
        result |= lane << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
