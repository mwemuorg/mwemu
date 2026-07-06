use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.rep.is_some() {
        if emu.rep.unwrap() == 0 || emu.cfg.verbose >= 3 {
            emu.show_instruction(
                color!("LightCyan"),
                &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
            );
        }
    } else {
        emu.show_instruction(
            color!("LightCyan"),
            &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        );
    }

    if emu.cfg.is_x64() {
        if !emu
            .maps
            .write_byte(emu.regs().rdi, emu.regs().get_al() as u8)
        {
            static FIRST_OOB: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            let rdi = emu.regs().rdi;
            if (0x412000..=0x420000).contains(&rdi)
                && !FIRST_OOB.swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                log::trace!(
                    "DEBUG first stosb OOB at rdi=0x{:x} rip=0x{:x} rcx=0x{:x} pos={}",
                    rdi,
                    emu.regs().rip,
                    emu.regs().rcx,
                    emu.pos,
                );
            }
            return false;
        }
        if emu.flag_df() {
            emu.regs_mut().rdi -= 1;
        } else {
            emu.regs_mut().rdi += 1;
        }
    } else {
        // 32bits
        if !emu
            .maps
            .write_byte(emu.regs().get_edi(), emu.regs().get_al() as u8)
        {
            return false;
        }
        if emu.flag_df() {
            let edi = emu.regs().get_edi() - 1;
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi() + 1;
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
