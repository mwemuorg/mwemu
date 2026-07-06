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

    let value0: u64 = match emu.maps.read_byte(emu.regs().rdi) {
        Some(value) => value.into(),
        None => {
            log::trace!("/!\\ error reading byte on rdi 0x{:x}", emu.regs().rdi);
            return false;
        }
    };

    let al = emu.regs().get_al();
    emu.flags_overwrite_mut().sub8(al, value0);

    if emu.cfg.is_x64() {
        if emu.flag_df() {
            emu.regs_mut().rdi -= 1;
        } else {
            emu.regs_mut().rdi += 1;
        }
    } else {
        // 32bits
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
