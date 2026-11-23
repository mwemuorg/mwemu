use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::{Instruction, Mnemonic};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let word = match ins.mnemonic() {
        Mnemonic::Fnstcw | Mnemonic::Fstcw => emu.fpu().get_ctrl(),
        Mnemonic::Fnstsw | Mnemonic::Fstsw => emu.fpu().status_word(),
        _ => return false,
    };
    emu.set_operand_value(ins, 0, u64::from(word))
}
