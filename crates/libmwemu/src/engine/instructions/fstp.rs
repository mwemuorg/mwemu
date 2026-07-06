use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, Register};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    log::trace!("count: {}", ins.op_count());
    if ins.op_register(0) == Register::ST0 {
        log::trace!("  param0 is st0");
    }
    if ins.op_kind(0) == iced_x86::OpKind::Memory {
        log::trace!("  param0 is mem");
        let res = emu.fpu_mut().get_st(0) as u64;

        if !emu.set_operand_value(ins, 0, res) {
            return false;
        }
    }
    if ins.op_count() > 1 && ins.op_kind(1) == iced_x86::OpKind::Memory {
        unreachable!("Fstp: param1 is mem");
    }

    emu.fpu_mut().pop_f64();
    emu.sync_fpu_ip();
    true
}
