use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, OpKind};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
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

    let value = match op_size {
        8 => match emu.stack_pop64(true) {
            Some(v) => v,
            None => {
                log::trace!("pop: cannot pop 64-bit value from stack");
                return false;
            }
        },
        4 => match emu.stack_pop32(true) {
            Some(v) => v as u64,
            None => {
                log::trace!("pop: cannot pop 32-bit value from stack");
                return false;
            }
        },
        _ => match emu.stack_pop16(true) {
            Some(v) => v as u64,
            None => {
                log::trace!("pop: cannot pop 16-bit value from stack");
                return false;
            }
        },
    };

    emu.show_instruction_pushpop(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        value,
    );

    if !emu.set_operand_value(ins, 0, value) {
        return false;
    }
    true
}
