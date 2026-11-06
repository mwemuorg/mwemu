use crate::emu;

pub fn GetCalendarWeekNumber(emu: &mut emu::Emu) {
    let systime_ptr = emu.regs().rcx;
    let cal_id = emu.regs().rdx as u32;
    let first_day = emu.regs().r8 as u32;
    let first_week = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!GetCalendarWeekNumber cal:{} firstday:{} firstweek:{}",
        cal_id,
        first_day,
        first_week
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
