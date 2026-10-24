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
        let val = match emu.maps.read_word(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::trace!("movsw: cannot read memory at rsi=0x{:x}", emu.regs().rsi);
                return false;
            }
        };
        emu.maps.write_word(emu.regs().rdi, val);

        if !emu.flag_df() {
            let v = emu.regs().rsi.wrapping_add(2);
            emu.regs_mut().rsi = v;
            let v = emu.regs().rdi.wrapping_add(2);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rsi.wrapping_sub(2);
            emu.regs_mut().rsi = v;
            let v = emu.regs().rdi.wrapping_sub(2);
            emu.regs_mut().rdi = v;
        }
    } else {
        // 32bits
        let val = match emu.maps.read_word(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                log::trace!(
                    "movsw: cannot read memory at esi=0x{:x}",
                    emu.regs().get_esi()
                );
                return false;
            }
        };
        emu.maps.write_word(emu.regs().get_edi(), val);

        if !emu.flag_df() {
            let esi = emu.regs().get_esi().wrapping_add(2);
            let edi = emu.regs().get_edi().wrapping_add(2);
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi().wrapping_sub(2);
            let edi = emu.regs().get_edi().wrapping_sub(2);
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
