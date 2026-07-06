use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    // the high part is cleared to zero

    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 128 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_xmm_value_128(ins, 0, value1);
    } else if sz0 == 128 && sz1 == 32 {
        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_xmm_value_128(ins, 0, value1 as u128);
    } else if sz0 == 32 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_value(ins, 0, value1 as u64);
    } else if sz0 == 128 && sz1 == 64 {
        let addr = match emu.get_operand_value(ins, 1, false) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm address value1");
                return false;
            }
        };
        let value1 = match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm qword value1");
                return false;
            }
        };

        emu.set_operand_xmm_value_128(ins, 0, value1 as u128);
    } else if sz0 == 64 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_value(ins, 0, value1 as u64);
    } else {
        log::trace!("SSE with other size combinations sz0:{} sz1:{}", sz0, sz1);
        return false;
    }
    true
}
