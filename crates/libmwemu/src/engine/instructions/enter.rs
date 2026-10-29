use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// ENTER size, level (SDM): push the frame pointer, copy the `level - 1`
/// outer frame pointers, link the new frame and reserve `size` bytes.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let size = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let level = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v & 0x1f,
        None => return false,
    };
    if emu.cfg.is_x64() {
        enter64(emu, size, level)
    } else {
        enter32(emu, size, level)
    }
}

fn enter64(emu: &mut Emu, size: u64, level: u64) -> bool {
    emu.stack_push64(emu.regs().rbp);
    let frame = emu.regs().rsp;
    if level > 0 {
        for _ in 1..level {
            let bp = emu.regs().rbp.wrapping_sub(8);
            emu.regs_mut().rbp = bp;
            let Some(outer) = emu.maps.read_qword(bp) else {
                return false;
            };
            emu.stack_push64(outer);
        }
        emu.stack_push64(frame);
    }
    emu.regs_mut().rbp = frame;
    let rsp = emu.regs().rsp.wrapping_sub(size);
    emu.regs_mut().rsp = rsp;
    true
}

fn enter32(emu: &mut Emu, size: u64, level: u64) -> bool {
    emu.stack_push32(emu.regs().get_ebp() as u32);
    let frame = emu.regs().get_esp();
    if level > 0 {
        for _ in 1..level {
            let bp = emu.regs().get_ebp().wrapping_sub(4) & 0xffff_ffff;
            emu.regs_mut().set_ebp(bp);
            let Some(outer) = emu.maps.read_dword(bp) else {
                return false;
            };
            emu.stack_push32(outer);
        }
        emu.stack_push32(frame as u32);
    }
    emu.regs_mut().set_ebp(frame);
    let esp = emu.regs().get_esp().wrapping_sub(size) & 0xffff_ffff;
    emu.regs_mut().set_esp(esp);
    true
}
