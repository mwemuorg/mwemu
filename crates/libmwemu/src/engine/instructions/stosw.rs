use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.cfg.is_x64() {
        let addr = emu.regs().rdi;
        let value = emu.regs().get_ax() as u16;
        let value = emu.call_memory_write_hook(addr, 16, value.into()) as u16;
        emu.maps.write_word(addr, value);

        if emu.flag_df() {
            let v = emu.regs().rdi.wrapping_sub(2);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rdi.wrapping_add(2);
            emu.regs_mut().rdi = v;
        }
    } else {
        // 32bits
        let addr = emu.regs().get_edi();
        let value = emu.regs().get_ax() as u16;
        let value = emu.call_memory_write_hook(addr, 16, value.into()) as u16;
        emu.maps.write_word(addr, value);

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
