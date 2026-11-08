use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstChangeNotificationW");
    emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationW");
    stdcall_ret(emu, 1);
}
