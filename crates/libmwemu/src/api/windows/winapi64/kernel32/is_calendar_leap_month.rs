use crate::emu;
use crate::winapi::helper;
pub fn IsCalendarLeapMonth(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let calendar = emu.regs().rdx;
    let year = emu.regs().r8;
    let month = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!IsCalendarLeapMonth locale:0x{:x} cal:{} year:{} month:{}",
        locale,
        calendar,
        year,
        month
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
