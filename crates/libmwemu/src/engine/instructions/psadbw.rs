use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for half in 0..2u32 {
        let mut sum = 0u32;
        for k in 0..8u32 {
            let bi = half * 8 + k;
            let d = ((dest >> (bi * 8)) & 0xff) as i32;
            let s = ((src >> (bi * 8)) & 0xff) as i32;
            sum += (d - s).unsigned_abs();
        }
        result |= (sum as u128) << (half * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
