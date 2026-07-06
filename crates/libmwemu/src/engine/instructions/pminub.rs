use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMINUB xmm1, xmm2/m128 : per-byte unsigned minimum, dest = min(dest, src).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let source1 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => {
            log::trace!("error reading xmm 0 operand");
            return false;
        }
    };

    let source2 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("error reading xmm/m128 1 operand");
            return false;
        }
    };

    let mut result: u128 = 0;
    for i in 0..16 {
        let byte1 = (source1 >> (8 * i)) & 0xFF;
        let byte2 = (source2 >> (8 * i)) & 0xFF;
        result |= byte1.min(byte2) << (8 * i);
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
