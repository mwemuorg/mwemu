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

    // Each output dword is the sum of two signed 16x16 products of adjacent
    // word lanes. The sum wraps (only 0x8000*0x8000 pairs overflow int32).
    let dwords = if emu.get_operand_sz(ins, 0) == 128 {
        4
    } else {
        2
    };
    let mut result: u128 = 0;
    for d in 0..dwords {
        let base = d * 32;
        let a0 = (((src0 >> base) & 0xFFFF) as u16) as i16 as i32;
        let a1 = (((src0 >> (base + 16)) & 0xFFFF) as u16) as i16 as i32;
        let b0 = (((src1 >> base) & 0xFFFF) as u16) as i16 as i32;
        let b1 = (((src1 >> (base + 16)) & 0xFFFF) as u16) as i16 as i32;
        let dword = (a0 * b0).wrapping_add(a1 * b1);
        result |= ((dword as u32) as u128) << base;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
