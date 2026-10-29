use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    debug_assert!(ins.op_count() == 2);

    let addr = match emu.get_operand_value(ins, 1, false) {
        Some(v) => v,
        None => {
            log::trace!("{} error getting movntdqa source address", emu.pos);
            return false;
        }
    };

    let value = match emu.maps.read_128bits_le(addr) {
        Some(v) => v,
        None => {
            log::trace!(
                "{} error reading movntdqa 128 bits at 0x{:x}",
                emu.pos,
                addr
            );
            return false;
        }
    };

    emu.set_operand_xmm_value_128(ins, 0, value);
    true
}
