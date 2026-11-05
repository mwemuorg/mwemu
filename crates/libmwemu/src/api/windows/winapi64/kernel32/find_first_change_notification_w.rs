use crate::emu;
use crate::winapi::helper;

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstChangeNotificationW");
    emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationW");
}
