use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    match emu.get_operand_sz(ins, 1) {
        128 => {
            let source1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 1 source operand");
                    return false;
                }
            };

            let mut result: u16 = 0;

            for i in 0..16 {
                let byte = ((source1 >> (i * 8)) & 0xff) as u16;
                let msb = (byte & 0x80) >> 7;
                result |= msb << i;
            }

            emu.set_operand_value(ins, 0, result as u64);
        }
        256 => {
            let source1 = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 1 source operand");
                    return false;
                }
            };

            let mut result: u32 = 0;
            let mut input_bytes = [0u8; 32];
            source1.to_little_endian(&mut input_bytes);

            for (i, byte) in input_bytes.iter().enumerate() {
                let msb = (byte & 0x80) >> 7;
                result |= (msb as u32) << i;
            }

            emu.set_operand_value(ins, 0, result as u64);
        }
        _ => unreachable!(""),
    }
    true
}
