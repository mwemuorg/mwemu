use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// PCMPGTW: signed word compare, `mm` (64-bit) and `xmm` (128-bit) forms.
/// Each lane of the destination becomes all ones where dest > src, else zero.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    // The MMX form shares the mnemonic: only the low 64 bits are lanes.
    let lanes = if ins.op_register(0).is_mm() {
        64 / 16
    } else {
        128 / 16
    };
    let mut result = 0u128;
    for i in 0..lanes {
        let shift = i * 16;
        let a = ((value0 >> shift) & 0xffff) as u16 as i16;
        let b = ((value1 >> shift) & 0xffff) as u16 as i16;
        if a > b {
            result |= 0xffffu128 << shift;
        }
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
