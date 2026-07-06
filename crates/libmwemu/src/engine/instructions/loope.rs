use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Yellow"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 1);

    let addr = match emu.get_jump_value(ins, 0) {
        Some(v) => v,
        None => return false,
    };

    if addr > 0xffffffff {
        if emu.regs_mut().rcx == 0 {
            emu.regs_mut().rcx = 0xffffffffffffffff;
        } else {
            emu.regs_mut().rcx -= 1;
        }

        if emu.regs().rcx > 0 && emu.flag_zf() {
            return emu.set_rip(addr, false);
        }
    } else if addr > 0xffff {
        if emu.regs_mut().get_ecx() == 0 {
            emu.regs_mut().set_ecx(0xffffffff);
        } else {
            let ecx = emu.regs().get_ecx() - 1;
            emu.regs_mut().set_ecx(ecx);
        }

        if emu.regs().get_ecx() > 0 && emu.flag_zf() {
            if emu.cfg.is_x64() {
                return emu.set_rip(addr, false);
            } else {
                return emu.set_eip(addr, false);
            }
        }
    } else {
        if emu.regs_mut().get_cx() == 0 {
            emu.regs_mut().set_cx(0xffff);
        } else {
            let cx = emu.regs().get_cx() - 1;
            emu.regs_mut().set_cx(cx);
        }

        if emu.regs().get_cx() > 0 && emu.flag_zf() {
            if emu.cfg.is_x64() {
                return emu.set_rip(addr, false);
            } else {
                return emu.set_eip(addr, false);
            }
        }
    }
    true
}
