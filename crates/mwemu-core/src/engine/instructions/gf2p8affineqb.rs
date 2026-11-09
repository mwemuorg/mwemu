use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let imm = try_operand!(emu.get_operand_value(ins, 2, true)) as u8;
    let mut result = 0u128;
    for j in 0..16u32 {
        let x = ((dest >> (j * 8)) & 0xff) as u8;
        let block = j / 8;
        let mut out = 0u8;
        for i in 0..8u32 {
            // Each source qword stores the matrix rows in reverse byte order.
            let row = ((src >> ((block * 8 + (7 - i)) * 8)) & 0xff) as u8;
            let bit = ((row & x).count_ones() & 1) as u8 ^ ((imm >> i) & 1);
            out |= bit << i;
        }
        result |= (out as u128) << (j * 8);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
