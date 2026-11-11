use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    let d = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let s = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let r3 = dw(d, 1) ^ dw(d, 3);
    let r2 = dw(d, 0) ^ dw(d, 2);
    let r1 = dw(s, 3) ^ dw(d, 1);
    let r0 = dw(s, 2) ^ dw(d, 0);
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (r0 as u128) | ((r1 as u128) << 32) | ((r2 as u128) << 64) | ((r3 as u128) << 96),
    )
}
