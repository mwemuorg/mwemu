use iced_x86::Instruction;

use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};

const ENV_FIELDS: usize = 7;
const SHORT_ENV_SIZE: usize = ENV_FIELDS * size_of::<u16>();
const OPCODE_MASK: u16 = (1 << 11) - 1;
const EXCEPTION_MASKS: u16 = (1 << 6) - 1;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let Some(addr) = emu.get_operand_value(ins, 0, false) else {
        return false;
    };
    let fpu = emu.fpu();
    let control = fpu.get_ctrl();
    // FNSTENV uses the 14/28-byte protected-mode format even in 64-bit mode.
    let mut bytes: Vec<u8> = if ins.memory_size().size() == SHORT_ENV_SIZE {
        [
            control,
            fpu.status,
            fpu.tag,
            fpu.ip as u16,
            fpu.code_segment,
            fpu.operand_ptr as u16,
            fpu.data_segment,
        ]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect()
    } else {
        [
            control as u32,
            fpu.status as u32,
            fpu.tag as u32,
            fpu.ip as u32,
            fpu.code_segment as u32 | (((fpu.opcode & OPCODE_MASK) as u32) << u16::BITS),
            fpu.operand_ptr as u32,
            fpu.data_segment as u32,
        ]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
    };
    if !emu.write_instruction_bytes(addr, &mut bytes) {
        return false;
    }
    emu.fpu_mut().set_ctrl(control | EXCEPTION_MASKS);
    true
}
