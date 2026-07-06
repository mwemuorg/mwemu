use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() == 2 {
        let sz0 = emu.get_operand_sz(ins, 0);

        if sz0 == 128 {
            let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
                Some(v) => v,
                None => {
                    log::trace!("error getting value0");
                    return false;
                }
            };
            let mut value1 = match emu.get_operand_value(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error getting value1");
                    return false;
                }
            };

            if value1 > 15 {
                value1 = 16;
            }

            let result: u128 = value0 >> (value1 * 8);

            emu.set_operand_xmm_value_128(ins, 0, result);
        } else {
            unimplemented!("size unimplemented");
        }
    } else if ins.op_count() == 3 {
        let sz0 = emu.get_operand_sz(ins, 0);

        if sz0 == 128 {
            let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error getting value0");
                    return false;
                }
            };
            let mut value2 = match emu.get_operand_value(ins, 2, true) {
                Some(v) => v,
                None => {
                    log::trace!("error getting value1");
                    return false;
                }
            };

            if value2 > 15 {
                value2 = 16;
            }

            let result: u128 = value1 >> (value2 * 8);

            emu.set_operand_xmm_value_128(ins, 0, result);
        } else {
            unimplemented!("size unimplemented");
        }
    } else {
        unreachable!();
    }
    true
}
