use crate::emu;
use crate::winapi::helper;
pub fn timeEndPeriod(emu: &mut emu::Emu) {
    let period = emu.regs().rcx;
    log_red!(emu, "kernel32!timeEndPeriod period:{}", period);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
