use crate::emu;

pub fn SetCalendarInfoW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;

    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "kernel32!SetCalendarInfoW name:'{}'", name);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
