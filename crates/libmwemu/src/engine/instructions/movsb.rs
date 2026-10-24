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
        let val = match emu.maps.read_byte(emu.regs().rsi) {
            Some(v) => v,
            None => {
                if emu.rep.is_some() {
                    log::trace!(
                        "rep movsb: unmapped read at rsi=0x{:x}, filling with zeros",
                        emu.regs().rsi
                    );
                    0u8
                } else {
                    log::trace!("cannot read memory on rsi");
                    return false;
                }
            }
        };
        if !emu.maps.write_byte(emu.regs().rdi, val) {
            static FIRST_OOB: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            let rdi = emu.regs().rdi;
            if (0x412000..=0x420000).contains(&rdi)
                && !FIRST_OOB.swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                log::trace!(
                    "DEBUG first movsb OOB rdi=0x{:x} rsi=0x{:x} rcx=0x{:x} rip=0x{:x} pos={}",
                    rdi,
                    emu.regs().rsi,
                    emu.regs().rcx,
                    emu.regs().rip,
                    emu.pos,
                );
            }
            log::trace!("cannot write memoryh on rdi");
            return false;
        }

        if !emu.flag_df() {
            let v = emu.regs().rsi.wrapping_add(1);
            emu.regs_mut().rsi = v;
            let v = emu.regs().rdi.wrapping_add(1);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rsi.wrapping_sub(1);
            emu.regs_mut().rsi = v;
            let v = emu.regs().rdi.wrapping_sub(1);
            emu.regs_mut().rdi = v;
        }
    } else {
        let val = match emu.maps.read_byte(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                if emu.rep.is_some() {
                    log::trace!(
                        "rep movsb: unmapped read at esi=0x{:x}, filling with zeros",
                        emu.regs().get_esi()
                    );
                    0u8
                } else {
                    log::trace!("cannot read memory on esi");
                    return false;
                }
            }
        };
        if !emu.maps.write_byte(emu.regs().get_edi(), val) {
            log::trace!("cannot write memory on edi");
            return false;
        }

        if !emu.flag_df() {
            let esi = emu.regs().get_esi().wrapping_add(1);
            let edi = emu.regs().get_edi().wrapping_add(1);
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi().wrapping_sub(1);
            let edi = emu.regs().get_edi().wrapping_sub(1);
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
