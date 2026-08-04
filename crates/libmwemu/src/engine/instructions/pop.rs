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
        8 => emu.stack_pop64(true).unwrap(),
        4 => emu.stack_pop32(true).unwrap() as u64,
        _ => emu.stack_pop16(true).unwrap() as u64, // the last one have to be 16 bit because there are only 64, 32 and 16 for pop instruction
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
