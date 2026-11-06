use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.rep.is_some() {
        if emu.rep.unwrap_or(0) == 0 || emu.cfg.verbose >= 3 {
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
        let addr = emu.regs().rdi;
        let value = emu.regs().get_eax() as u32;
        let value = emu.call_memory_write_hook(addr, 32, value.into()) as u32;
        if !emu.maps.write_dword(addr, value) {
            return false;
        }
        if emu.flag_df() {
            let v = emu.regs().rdi.wrapping_sub(4);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rdi.wrapping_add(4);
            emu.regs_mut().rdi = v;
        }
    } else {
        // 32bits
        let addr = emu.regs().get_edi();
        let value = emu.regs().get_eax() as u32;
        let value = emu.call_memory_write_hook(addr, 32, value.into()) as u32;
        if !emu.maps.write_dword(addr, value) {
            return false;
        }

        if emu.flag_df() {
            let edi = emu.regs().get_edi().wrapping_sub(4);
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi().wrapping_add(4);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
