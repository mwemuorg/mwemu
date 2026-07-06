use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 128 && sz1 == 128 {
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
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
            let qword0 = (value0 >> (64 * i)) & 0xFFFFFFFFFFFFFFFF;
            let qword1 = (value1 >> (64 * i)) & 0xFFFFFFFFFFFFFFFF;
            let res_qword = qword0.wrapping_sub(qword1);
            result |= res_qword << (64 * i);
        }

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        unimplemented!();
    }
    true
}
