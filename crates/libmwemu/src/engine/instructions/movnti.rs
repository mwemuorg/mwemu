use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    debug_assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => {
            emu.show_instruction_comment(
                color!("LightCyan"),
                &DecodedInstruction::X86(*ins),
                "error",
            );
            return false;
        }
    };
    emu.show_instruction_comment(
        color!("LightCyan"),
        &DecodedInstruction::X86(*ins),
        &format!("0x{:x}", value1),
    );

    emu.set_operand_value(ins, 0, value1)
}
