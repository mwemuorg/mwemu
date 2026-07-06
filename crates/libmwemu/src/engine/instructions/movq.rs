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
    let sz1 = emu.get_operand_sz(ins, 1);
    let value1: u128;

    if sz1 == 128 {
        value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
    } else if sz1 < 128 {
        value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v as u128,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
    } else {
        unimplemented!("ymm zmm unimplemented on movq");
    }

    if sz0 == 128 {
        emu.set_operand_xmm_value_128(ins, 0, value1);
    } else if sz0 < 128 {
        emu.set_operand_value(ins, 0, value1 as u64);
    } else {
        unimplemented!("ymm zmm unimplemented on movq");
    }
    true
}
