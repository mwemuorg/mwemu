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

        let mut result = 0u128;
        for i in 0..2 {
            let word_value0 = (value0 >> (i * 16)) & 0xFFFF;
            let word_value1 = (value1 >> (i * 16)) & 0xFFFF;
            result |= word_value0 << (i * 32);
            result |= word_value1 << (i * 32 + 16);
        }

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        unimplemented!("unimplemented size");
    }
    true
}
