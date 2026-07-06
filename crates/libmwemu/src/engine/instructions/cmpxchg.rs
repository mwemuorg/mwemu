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

    // CMPXCHG dest, src: compare the accumulator with dest.
    //   equal    -> ZF=1, dest = src
    //   not equal -> ZF=0, accumulator = dest  (the *destination* value, NOT src)
    // Loading src on the mismatch path breaks lock retry loops (e.g. glibc's
    // rwlock CAS-retry), which then mis-read the lock state and deadlock.
    if emu.cfg.is_x64() {
        if value0 == emu.regs().rax {
            emu.flags_mut().f_zf = true;
            if !emu.set_operand_value(ins, 0, value1) {
                return false;
            }
        } else {
            emu.flags_mut().f_zf = false;
            emu.regs_mut().rax = value0;
        }
    } else {
        // 32bits
        if value0 == emu.regs().get_eax() {
            emu.flags_mut().f_zf = true;
            if !emu.set_operand_value(ins, 0, value1) {
                return false;
            }
        } else {
            emu.flags_mut().f_zf = false;
            emu.regs_mut().set_eax(value0);
        }
    }
    true
}
