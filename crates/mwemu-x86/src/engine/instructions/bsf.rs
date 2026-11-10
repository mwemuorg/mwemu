use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    debug_assert!(ins.op_count() == 2);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);

    if value1 == 0 {
        emu.flags_mut().f_zf = true;

        if emu.cfg.verbose >= 1 {
            log::trace!("/!\\ undefined behavior on BSF with src == 0");
        }
    } else {
        emu.flags_mut().f_zf = false;

        if !emu.set_operand_value(ins, 0, value1.trailing_zeros() as u64) {
            return false;
        }
    }

    // cf flag undefined behavior apple mac x86_64 problem
    if emu.regs_mut().rip == 0x144ed424a {
        if emu.cfg.verbose >= 1 {
            log::trace!("/!\\ f_cf undefined behaviour");
        }
        emu.flags_mut().f_cf = false;
    }

    true
}
