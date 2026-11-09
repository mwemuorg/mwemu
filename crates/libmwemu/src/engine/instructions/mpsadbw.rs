use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let imm = try_operand!(emu.get_operand_value(ins, 2, true)) as u32;
    let src_off = ((imm >> 2) & 1) * 4;
    let dst_off = (imm & 3) * 4;
    let byte = |v: u128, i: u32| -> i32 {
        if i < 16 {
            ((v >> (i * 8)) & 0xff) as i32
        } else {
            0
        }
    };
    let mut result = 0u128;
    // Slide the four-byte destination window against a fixed source block.
    for j in 0..8u32 {
        let mut sum = 0u32;
        for k in 0..4u32 {
            sum += (byte(dest, dst_off + j + k) - byte(src, src_off + k)).unsigned_abs();
        }
        result |= (sum as u128) << (j * 16);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
