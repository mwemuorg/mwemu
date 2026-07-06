use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let value0: u32;
    let value1: u32;

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
        value0 = match emu.maps.read_dword(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::trace!("cannot read rsi");
                return false;
            }
        };
        value1 = match emu.maps.read_dword(emu.regs().rdi) {
            Some(v) => v,
            None => {
                log::trace!("cannot read rdi");
                return false;
            }
        };

        if emu.flag_df() {
            emu.regs_mut().rsi -= 4;
            emu.regs_mut().rdi -= 4;
        } else {
            emu.regs_mut().rsi += 4;
            emu.regs_mut().rdi += 4;
        }
    } else {
        // 32bits
        value0 = match emu.maps.read_dword(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                log::trace!("cannot read esi");
                return false;
            }
        };
        value1 = match emu.maps.read_dword(emu.regs().get_edi()) {
            Some(v) => v,
            None => {
                log::trace!("cannot read edi");
                return false;
            }
        };

        if emu.flag_df() {
            let esi = emu.regs().get_esi() - 4;
            let edi = emu.regs().get_edi() - 4;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi() + 4;
            let edi = emu.regs().get_edi() + 4;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }

    emu.flags_overwrite_mut()
        .sub32(value0 as u64, value1 as u64);

    if emu.cfg.verbose >= 2 {
        if value0 > value1 {
            log::trace!("\tcmp: 0x{:x} > 0x{:x}", value0, value1);
        } else if value0 < value1 {
            log::trace!("\tcmp: 0x{:x} < 0x{:x}", value0, value1);
        } else {
            log::trace!("\tcmp: 0x{:x} == 0x{:x}", value0, value1);
        }
    }
    true
}
