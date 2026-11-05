use crate::emu;

pub fn IsCalendarLeapYear(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let calendar = emu.regs().rdx;
    let year = emu.regs().r8;
    let era = emu.regs().r9;
    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    log_red!(
        emu,
        "kernel32!IsCalendarLeapYear locale:0x{:x} cal:{} year:{} era:{} => {}",
        locale,
        calendar,
        year,
        era,
        is_leap
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_leap { 1 } else { 0 };
}
