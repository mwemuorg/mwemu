use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);
    let sz0 = emu.get_operand_sz(ins, 0);
    if sz0 == 128 {
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value0");
                return false;
            }
        };
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };

        let mut result: u128 = 0;
        let mask_byte = 0xff;

        for i in 0..8 {
            let byte_value0 = (value0 >> (8 * i)) & mask_byte;
            let byte_value1 = (value1 >> (8 * i)) & mask_byte;

            result |= byte_value0 << (16 * i);
            result |= byte_value1 << (16 * i + 8);
        }

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        unimplemented!("unimplemented size");
    }
    true
}
