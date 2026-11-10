use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    debug_assert!(ins.op_count() == 2);

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    let value: u64 = if sz1 == 128 || sz1 == 256 {
        try_operand!(emu.get_operand_xmm_value_128(ins, 1, true)) as u64
    } else {
        match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error reading second operand");
                return false;
            }
        }
    };

    match sz0 {
        64 => emu.set_operand_value(ins, 0, value),
        128 => emu.set_operand_xmm_value_128(ins, 0, value as u128),
        256 => emu.set_operand_ymm_value_256(ins, 0, regs64::U256::from(value)),
        _ => {
            log::warn!("vmovq: unsupported operand size {}", sz0);
            false
        }
    }
}
