use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    debug_assert!(ins.op_count() == 2);

    let sz0 = emu.get_operand_sz(ins, 0);

    if sz0 == 32 {
        let xmm = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
        return emu.set_operand_value(ins, 0, (xmm as u32) as u64);
    }

    let value = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v as u32 as u128,
        None => {
            log::trace!("error reading second operand");
            return false;
        }
    };

    match sz0 {
        128 => emu.set_operand_xmm_value_128(ins, 0, value),
        256 => emu.set_operand_ymm_value_256(ins, 0, regs64::U256::from(value as u64)),
        _ => false,
    }
}
