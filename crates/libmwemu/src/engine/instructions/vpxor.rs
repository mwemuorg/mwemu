use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    match emu.get_operand_sz(ins, 0) {
        128 => {
            let source1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 1 source operand");
                    return false;
                }
            };

            let source2 = match emu.get_operand_xmm_value_128(ins, 2, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 2 source operand");
                    return false;
                }
            };

            emu.set_operand_xmm_value_128(ins, 0, source1 ^ source2);
        }
        256 => {
            let source1 = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 1 source operand");
                    return false;
                }
            };

            let source2 = match emu.get_operand_ymm_value_256(ins, 2, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 2 source operand");
                    return false;
                }
            };

            emu.set_operand_ymm_value_256(ins, 0, source1 ^ source2);
        }
        _ => unreachable!(""),
    }
    true
}
