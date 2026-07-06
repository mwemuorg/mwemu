use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => {
            log::trace!("error getting value0");
            return false;
        }
    };
    let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("error getting value1");
            return false;
        }
    };

    let a: u128 = (value0 & 0xffffffff) + (value1 & 0xffffffff);
    let b: u128 = (value0 & 0xffffffff_00000000) + (value1 & 0xffffffff_00000000);
    let c: u128 = (value0 & 0xffffffff_00000000_00000000) + (value1 & 0xffffffff_00000000_00000000);
    let d: u128 = (value0 & 0xffffffff_00000000_00000000_00000000)
        + (value1 & 0xffffffff_00000000_00000000_00000000);

    let result: u128 = a | b | c | d;

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
