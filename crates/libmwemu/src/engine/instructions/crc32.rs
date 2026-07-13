use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CRC32 dest, src: accumulate the CRC-32C (Castagnoli, reversed poly 0x82F63B78)
// of the source bytes into the destination.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let dest = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let nbytes = (emu.get_operand_sz(ins, 1) / 8) as u32;

    let mut crc = dest as u32;
    for i in 0..nbytes {
        crc ^= ((src >> (i * 8)) & 0xff) as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0x82F63B78u32 & (crc & 1).wrapping_neg());
        }
    }

    if !emu.set_operand_value(ins, 0, crc as u64) {
        return false;
    }
    true
}
