use crate::emu;
use crate::winapi::helper;
pub fn timeBeginPeriod(emu: &mut emu::Emu) {
    let period = emu.regs().rcx;
    log_red!(emu, "kernel32!timeBeginPeriod period:{}", period);
    let handle = helper::handler_create("TimeBeginPeriod");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
