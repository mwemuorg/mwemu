use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, OpKind};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Orange"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let acc = match sz {
        8 => emu.regs().get_al(),
        16 => emu.regs().get_ax(),
        32 => emu.regs().get_eax(),
        _ => emu.regs().rax,
    };

    if acc == value0 || ins.op0_kind() == OpKind::Memory {
        let value = if acc == value0 { value1 } else { value0 };
        if !emu.set_operand_value(ins, 0, value) {
            return false;
        }
    }

    match sz {
        8 => emu.flags_overwrite_mut().sub8(acc, value0),
        16 => emu.flags_overwrite_mut().sub16(acc, value0),
        32 => emu.flags_overwrite_mut().sub32(acc, value0),
        _ => emu.flags_overwrite_mut().sub64(acc, value0),
    };

    // On mismatch, return the previous destination in the accumulator so retry loops
    // observe the contested value. Loading the source here can deadlock those loops.
    if acc != value0 {
        match sz {
            8 => emu.regs_mut().set_al(value0),
            16 => emu.regs_mut().set_ax(value0),
            32 => emu.regs_mut().set_eax(value0),
            _ => emu.regs_mut().rax = value0,
        }
    }
    true
}
