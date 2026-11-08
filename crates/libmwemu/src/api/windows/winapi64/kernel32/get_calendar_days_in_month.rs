use crate::emu;

pub fn GetCalendarDaysInMonth(emu: &mut emu::Emu) {
    let calendar = emu.regs().rcx as u32;
    let year = emu.regs().rdx as u32;
    let month = emu.regs().r8 as u32;
    log_red!(
        emu,
        "kernel32!GetCalendarDaysInMonth cal:{} year:{} month:{}",
        calendar,
        year,
        month
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = days;
}
