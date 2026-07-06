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

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let eax = emu.regs().get_eax();
    emu.flags_overwrite_mut().sub32(eax, value0);

    if emu.cfg.is_x64() {
        if emu.flag_df() {
            emu.regs_mut().rdi -= 4;
        } else {
            emu.regs_mut().rdi += 4;
        }
    } else {
        // 32bits
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
