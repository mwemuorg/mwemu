use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    //TODO: exception if memory address is unaligned to 16,32,64

    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);
    let sz_max = sz0.max(sz1);

    match sz_max {
        128 => {
            let source = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 1 source operand");
                    return false;
                }
            };

            emu.set_operand_xmm_value_128(ins, 0, source);
        }
        256 => {
            let source = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 1 source operand");
                    return false;
                }
            };

            emu.set_operand_ymm_value_256(ins, 0, source);
        }
        _ => unimplemented!("unimplemented operand size"),
    }
    true
}
