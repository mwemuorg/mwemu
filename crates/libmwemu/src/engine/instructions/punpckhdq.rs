use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

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

    let dword0_0 = (value0 >> 96) as u32;
    let dword0_1 = (value0 >> 64) as u32;
    let dword1_0 = (value1 >> 96) as u32;
    let dword1_1 = (value1 >> 64) as u32;

    let result: u128 = ((dword0_0 as u128) << 96)
        | ((dword1_0 as u128) << 64)
        | ((dword0_1 as u128) << 32)
        | (dword1_1 as u128);

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
