use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let src0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);

    let mut result = [0i32; 2];

    for i in 0..4 {
        let shift = i * 16;
        let a = ((src0 >> shift) & 0xFFFF) as i16 as i32;
        let b = ((src1 >> shift) & 0xFFFF) as i16 as i32;

        let product = a * b;

        if i < 2 {
            result[0] += product;
        } else {
            result[1] += product;
        }
    }

    let final_result = ((result[1] as u64) << 32) | (result[0] as u64);

    emu.set_operand_xmm_value_128(ins, 0, final_result as u128);
    true
}
