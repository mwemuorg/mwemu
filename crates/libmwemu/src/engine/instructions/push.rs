use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, OpKind};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let value = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    emu.show_instruction_pushpop(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        value,
    );

    let op_size = match ins.op_kind(0) {
        OpKind::Register => ins.op_register(0).size(),
        OpKind::Memory => ins.memory_size().size(),
        // this is the case of immediate value then it is depend on the stack addr size which is the execution architecture
        _ => {
            if emu.cfg.is_x64() {
                8
            } else {
                4
            }
        }
    };

    let result = match op_size {
        8 => emu.stack_push64(value),
        4 => emu.stack_push32(value as u32),
        _ => emu.stack_push16(value as u16), // the last one have to be 16 bit because there are only 64, 32 and 16 for pop instruction
    };

    result
}
