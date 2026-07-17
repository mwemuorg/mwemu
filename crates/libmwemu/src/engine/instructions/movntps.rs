use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("error getting movntps source xmm value");
            return false;
        }
    };

    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v,
        None => {
            log::trace!("error getting movntps destination address");
            return false;
        }
    };

    if !emu.maps.write_128bits_le(addr, value1) {
        log::trace!("error writing movntps 128 bits at 0x{:x}", addr);
        return false;
    }
    true
}
