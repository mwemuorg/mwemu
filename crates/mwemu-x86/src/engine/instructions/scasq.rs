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
    let value0: u64 = match emu.maps.read_qword(di) {
        Some(v) => v,
        None => {
            log::trace!("/!\\ error reading qword on rdi 0x{:x}", di);
            return false;
        }
    };

    let rax = emu.regs().rax;
    emu.flags_overwrite_mut().sub64(rax, value0);

    if emu.flag_df() {
        let v = emu.regs().rdi.wrapping_sub(8);
        emu.regs_mut().rdi = v;
    } else {
        let v = emu.regs().rdi.wrapping_add(8);
        emu.regs_mut().rdi = v;
    }
    true
}
