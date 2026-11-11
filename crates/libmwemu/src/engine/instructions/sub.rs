use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));

    debug_assert!(ins.op_count() == 2);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let res: u64 = match emu.get_operand_sz(ins, 0) {
        64 => emu.flags_overwrite_mut().sub64(value0, value1),
        32 => emu.flags_overwrite_mut().sub32(value0, value1),
        16 => emu.flags_overwrite_mut().sub16(value0, value1),
        8 => emu.flags_overwrite_mut().sub8(value0, value1),
        _ => {
            log::warn!("sub: unexpected operand size");
            return false;
        }
    };

    emu.set_operand_value(ins, 0, res)
}
