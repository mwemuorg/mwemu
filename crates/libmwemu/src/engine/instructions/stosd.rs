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
            log::trace!("    rdi: 0x{:x}", emu.regs().rdi);
        }
    } else {
        emu.show_instruction(
            color!("LightCyan"),
            &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        );
        log::trace!("    rdi: 0x{:x}", emu.regs().rdi);
    }

    if emu.cfg.is_x64() {
        if !emu
            .maps
            .write_dword(emu.regs().rdi, emu.regs().get_eax() as u32)
        {
            return false;
        }
        if emu.flag_df() {
            emu.regs_mut().rdi -= 4;
        } else {
            emu.regs_mut().rdi += 4;
        }
    } else {
        // 32bits
        if !emu
            .maps
            .write_dword(emu.regs().get_edi(), emu.regs().get_eax() as u32)
        {
            return false;
        }

        if emu.flag_df() {
            let edi = emu.regs().get_edi() - 4;
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi() + 4;
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
