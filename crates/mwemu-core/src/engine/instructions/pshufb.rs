use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let dest = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let ctrl = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let src_bytes = dest.to_le_bytes();
    let ctrl_bytes = ctrl.to_le_bytes();
    let mut result = [0u8; 16];
    let bytes = (emu.get_operand_sz(ins, 0) / u8::BITS) as usize;
    for i in 0..bytes {
        let c = ctrl_bytes[i];
        if c & (1 << (u8::BITS - 1)) == 0 {
            result[i] = src_bytes[c as usize & (bytes - 1)];
        }
    }

    emu.set_operand_xmm_value_128(ins, 0, u128::from_le_bytes(result))
}
