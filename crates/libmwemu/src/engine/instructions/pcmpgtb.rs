use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);
    assert!(emu.get_operand_sz(ins, 0) == 128);
    assert!(emu.get_operand_sz(ins, 1) == 128);

    let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let mut result = 0u128;

    for i in 0..16 {
        let shift = i * 8;
        let byte0 = (value0 >> shift) & 0xFF;
        let byte1 = (value1 >> shift) & 0xFF;

        let cmp_result = if byte0 > byte1 { 0xFFu128 } else { 0x00u128 };

        result |= cmp_result << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
