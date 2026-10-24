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

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
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
