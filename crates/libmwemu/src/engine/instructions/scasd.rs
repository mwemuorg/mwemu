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

    // Operand 0 is the accumulator in iced; the string element lives at [rdi].
    let di = if emu.cfg.is_x64() {
        emu.regs().rdi
    } else {
        emu.regs().get_edi()
    };
    let value0: u64 = match emu.maps.read_dword(di) {
        Some(v) => v.into(),
        None => {
            log::trace!("/!\\ error reading dword on rdi 0x{:x}", di);
            return false;
        }
    };

    let eax = emu.regs().get_eax();
    emu.flags_overwrite_mut().sub32(eax, value0);

    if emu.cfg.is_x64() {
        if emu.flag_df() {
            let v = emu.regs().rdi.wrapping_sub(4);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rdi.wrapping_add(4);
            emu.regs_mut().rdi = v;
        }
    } else {
        // 32bits
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
