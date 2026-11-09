use crate::emu;
use crate::winapi::helper;

pub(super) fn GetProcessWindowStation(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetProcessWindowStation");
    emu.regs_mut().rax = helper::handler_create("WinSta0");
}
