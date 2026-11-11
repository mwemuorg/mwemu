use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));

    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let dest = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let nbytes = emu.get_operand_sz(ins, 1) / 8;

    let mut crc = dest as u32;
    for i in 0..nbytes {
        crc ^= ((src >> (i * 8)) & 0xff) as u32;
        for _ in 0..8 {
            // Reversed Castagnoli polynomial (CRC-32C).
            crc = (crc >> 1) ^ (0x82F63B78u32 & (crc & 1).wrapping_neg());
        }
    }

    emu.set_operand_value(ins, 0, crc as u64)
}
