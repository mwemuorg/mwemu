use crate::emu;

pub fn CompareCalendarDates(emu: &mut emu::Emu) {
    let p_date1 = emu.regs().rcx;
    let p_date2 = emu.regs().rdx;
    let cal_id = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!CompareCalendarDates date1:0x{:x} date2:0x{:x} calId:{}",
        p_date1,
        p_date2,
        cal_id
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
}
