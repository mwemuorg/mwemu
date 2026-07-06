use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let tmp_sp = emu.regs().get_sp() as u16;

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_ax() as u16)
    {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_cx() as u16)
    {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_dx() as u16)
    {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_bx() as u16)
    {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu.maps.write_word(emu.regs().get_sp(), tmp_sp) {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_bp() as u16)
    {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_si() as u16)
    {
        return false;
    }

    emu.regs_mut().sub_sp();
    if !emu
        .maps
        .write_word(emu.regs().get_sp(), emu.regs().get_di() as u16)
    {
        return false;
    }

    true
}
