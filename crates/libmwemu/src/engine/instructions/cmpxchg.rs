use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

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

    // CMPXCHG dest, src: compare the accumulator (AL/AX/EAX/RAX, matching the
    // operand size) with dest.
    //   equal     -> dest = src
    //   not equal -> accumulator = dest  (the *destination* value, NOT src)
    // Loading src on the mismatch path breaks lock retry loops (e.g. glibc's
    // rwlock CAS-retry), which then mis-read the lock state and deadlock.
    // All the status flags are set from the comparison, exactly like CMP.
    let sz = emu.get_operand_sz(ins, 0);
    let acc = match sz {
        8 => emu.regs().get_al(),
        16 => emu.regs().get_ax(),
        32 => emu.regs().get_eax(),
        _ => emu.regs().rax,
    };

    match sz {
        8 => emu.flags_overwrite_mut().sub8(acc, value0),
        16 => emu.flags_overwrite_mut().sub16(acc, value0),
        32 => emu.flags_overwrite_mut().sub32(acc, value0),
        _ => emu.flags_overwrite_mut().sub64(acc, value0),
    };

    if acc == value0 {
        if !emu.set_operand_value(ins, 0, value1) {
            return false;
        }
    } else {
        match sz {
            8 => emu.regs_mut().set_al(value0),
            16 => emu.regs_mut().set_ax(value0),
            32 => emu.regs_mut().set_eax(value0),
            _ => emu.regs_mut().rax = value0,
        }
    }
    true
}
