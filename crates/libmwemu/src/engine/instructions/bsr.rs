use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let sz = emu.get_operand_sz(ins, 0);
    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);

    if value1 == 0 {
        if emu.cfg.verbose >= 3 {
            log::trace!("/!\\ bsr src == 0 is undefined behavior");
        }
        emu.flags_mut().f_zf = true;
        return true;
    }

    let result = match sz {
        64 => 63 - value1.leading_zeros() as u64,
        32 => 31 - (value1 as u32).leading_zeros() as u64,
        16 => 15 - (value1 as u16).leading_zeros() as u64,
        8 => 7 - (value1 as u8).leading_zeros() as u64,
        _ => return false,
    };

    emu.flags_mut().f_zf = false;

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
