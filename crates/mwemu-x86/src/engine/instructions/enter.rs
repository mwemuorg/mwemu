use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));

    let size = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let level = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v & 0x1f,
        None => return false,
    };
    let sp = emu.sp();
    let bp = emu.regs().rbp;
    let ok = if emu.cfg.is_x64() {
        enter64(emu, size, level)
    } else {
        enter32(emu, size, level)
    };
    if !ok {
        emu.set_sp(sp);
        emu.regs_mut().rbp = bp;
    }
    ok
}

fn enter64(emu: &mut Emu, size: u64, level: u64) -> bool {
    if !emu.push_instruction(emu.regs().rbp, u64::BITS) {
        return false;
    }
    let frame = emu.regs().rsp;
    if level > 0 {
        for _ in 1..level {
            let bp = emu.regs().rbp.wrapping_sub(8);
            emu.regs_mut().rbp = bp;
            let Some(outer) = emu.maps.read_qword(bp) else {
                return false;
            };
            if !emu.push_instruction(outer, u64::BITS) {
                return false;
            }
        }
        if !emu.push_instruction(frame, u64::BITS) {
            return false;
        }
    }
    emu.regs_mut().rbp = frame;
    let rsp = emu.regs().rsp.wrapping_sub(size);
    emu.regs_mut().rsp = rsp;
    true
}

fn enter32(emu: &mut Emu, size: u64, level: u64) -> bool {
    if !emu.push_instruction(emu.regs().get_ebp(), u32::BITS) {
        return false;
    }
    let frame = emu.regs().get_esp();
    if level > 0 {
        for _ in 1..level {
            let bp = emu.regs().get_ebp().wrapping_sub(4) & 0xffff_ffff;
            emu.regs_mut().set_ebp(bp);
            let Some(outer) = emu.maps.read_dword(bp) else {
                return false;
            };
            if !emu.push_instruction(outer as u64, u32::BITS) {
                return false;
            }
        }
        if !emu.push_instruction(frame, u32::BITS) {
            return false;
        }
    }
    emu.regs_mut().set_ebp(frame);
    let esp = emu.regs().get_esp().wrapping_sub(size) & 0xffff_ffff;
    emu.regs_mut().set_esp(esp);
    true
}
