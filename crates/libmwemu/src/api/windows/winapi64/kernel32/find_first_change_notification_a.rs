use crate::emu;
use crate::winapi::helper;

pub fn FindFirstChangeNotificationA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstChangeNotificationA");
    emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationA");
}
