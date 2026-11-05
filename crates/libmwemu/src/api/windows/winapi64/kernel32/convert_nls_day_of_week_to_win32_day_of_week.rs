use crate::emu;

pub fn ConvertNLSDayOfWeekToWin32DayOfWeek(emu: &mut emu::Emu) {
    let nls_day = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!ConvertNLSDayOfWeekToWin32DayOfWeek nlsDay:{}",
        nls_day
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = (nls_day + 1) % 7;
}
