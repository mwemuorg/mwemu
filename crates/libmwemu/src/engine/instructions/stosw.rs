use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.cfg.is_x64() {
        emu.maps
            .write_word(emu.regs().rdi, emu.regs().get_ax() as u16);

        if emu.flag_df() {
            let v = emu.regs().rdi.wrapping_sub(2);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rdi.wrapping_add(2);
            emu.regs_mut().rdi = v;
        }
    } else {
        // 32bits
        emu.maps
            .write_word(emu.regs().get_edi(), emu.regs().get_ax() as u16);

        if emu.flag_df() {
            let edi = emu.regs().get_edi().wrapping_sub(2);
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi().wrapping_add(2);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
