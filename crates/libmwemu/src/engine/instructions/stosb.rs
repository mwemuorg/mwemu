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
        }
    } else {
        emu.show_instruction(
            color!("LightCyan"),
            &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        );
    }

    if emu.cfg.is_x64() {
        let addr = emu.regs().rdi;
        let value = emu.regs().get_al() as u8;
        let value = emu.call_memory_write_hook(addr, 8, value.into()) as u8;
        if !emu.maps.write_byte(addr, value) {
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
            let v = emu.regs().rdi.wrapping_sub(1);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rdi.wrapping_add(1);
            emu.regs_mut().rdi = v;
        }
    } else {
        let addr = emu.regs().get_edi();
        let value = emu.regs().get_al() as u8;
        let value = emu.call_memory_write_hook(addr, 8, value.into()) as u8;
        if !emu.maps.write_byte(addr, value) {
            return false;
        }
        if emu.flag_df() {
            let edi = emu.regs().get_edi().wrapping_sub(1);
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi().wrapping_add(1);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
