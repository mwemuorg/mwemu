use crate::emu;

pub fn GetCalendarMonthsInYear(emu: &mut emu::Emu) {
    let calendar = emu.regs().rcx as u32;
    let year = emu.regs().rdx as u32;
    log_red!(
        emu,
        "kernel32!GetCalendarMonthsInYear cal:{} year:{}",
        calendar,
        year
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 12;
}
