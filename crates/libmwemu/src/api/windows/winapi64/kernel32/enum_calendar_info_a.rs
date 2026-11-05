use crate::emu;

pub fn EnumCalendarInfoA(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;
    log_red!(emu, "kernel32!EnumCalendarInfoA callback:0x{:x}", callback);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
