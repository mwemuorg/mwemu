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
    emu.pos += 1;

    debug_assert!(emu.cfg.is_x64());

    let val = match emu.maps.read_qword(emu.regs().rsi) {
        Some(v) => v,
        None => {
            log::trace!("movsq: cannot read memory at rsi=0x{:x}", emu.regs().rsi);
            return false;
        }
    };
    emu.maps.write_qword(emu.regs().rdi, val);

    if !emu.flag_df() {
        let v = emu.regs().rsi.wrapping_add(8);
        emu.regs_mut().rsi = v;
        let v = emu.regs().rdi.wrapping_add(8);
        emu.regs_mut().rdi = v;
    } else {
        let v = emu.regs().rsi.wrapping_sub(8);
        emu.regs_mut().rsi = v;
        let v = emu.regs().rdi.wrapping_sub(8);
        emu.regs_mut().rdi = v;
    }
    true
}
