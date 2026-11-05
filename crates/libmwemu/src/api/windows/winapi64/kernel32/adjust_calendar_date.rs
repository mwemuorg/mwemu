use crate::emu;

pub fn AdjustCalendarDate(emu: &mut emu::Emu) {
    let _CALDATETIME = emu.regs().rcx as u32;
    let _int = emu.regs().rdx as u32;
    let _int = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!AdjustCalendarDate");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
