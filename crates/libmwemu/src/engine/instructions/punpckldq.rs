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

    let dword0_0 = (value0 & 0xFFFFFFFF) as u32;
    let dword0_1 = ((value0 >> 32) & 0xFFFFFFFF) as u32;
    let dword1_0 = (value1 & 0xFFFFFFFF) as u32;
    let dword1_1 = ((value1 >> 32) & 0xFFFFFFFF) as u32;

    // PUNPCKLDQ interleaves the low dwords: result = [dest0, src0, dest1, src1]
    // from the low end up.
    let result: u128 = (dword0_0 as u128)
        | ((dword1_0 as u128) << 32)
        | ((dword0_1 as u128) << 64)
        | ((dword1_1 as u128) << 96);

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
