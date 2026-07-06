use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, Register};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    match ins.op_register(0) {
        Register::ST0 => emu.fpu_mut().clear_st(0),
        Register::ST1 => emu.fpu_mut().clear_st(1),
        Register::ST2 => emu.fpu_mut().clear_st(2),
        Register::ST3 => emu.fpu_mut().clear_st(3),
        Register::ST4 => emu.fpu_mut().clear_st(4),
        Register::ST5 => emu.fpu_mut().clear_st(5),
        Register::ST6 => emu.fpu_mut().clear_st(6),
        Register::ST7 => emu.fpu_mut().clear_st(7),
        _ => unimplemented!("impossible case"),
    }

    emu.sync_fpu_ip();
    true
}
