use crate::emu;
pub fn IsValidCalDateTime(emu: &mut emu::Emu) {
    let lp_cal_date_time = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!IsValidCalDateTime cal:0x{:x} flags:0x{:x} => TRUE",
        lp_cal_date_time,
        flags
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
