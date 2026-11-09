use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let d = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let s = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let g = |v: u128, n: u32| (v >> (32 * n)) & 0xffffffff;
    let r = g(d, 2) | (g(s, 2) << 32) | (g(d, 3) << 64) | (g(s, 3) << 96);
    emu.set_operand_xmm_value_128(ins, 0, r)
}
