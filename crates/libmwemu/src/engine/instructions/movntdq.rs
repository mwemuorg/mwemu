use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    debug_assert!(ins.op_count() == 2);

    let value = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("{} error getting movntdq source xmm value", emu.pos);
            return false;
        }
    };

    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v,
        None => {
            log::trace!("{} error getting movntdq destination address", emu.pos);
            return false;
        }
    };

    if !emu.maps.write_128bits_le(addr, value) {
        log::trace!("{} error writing movntdq 128 bits at 0x{:x}", emu.pos, addr);
        return false;
    }
    true
}
