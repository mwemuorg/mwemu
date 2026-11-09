use crate::emu;
use crate::winapi::helper;

pub fn UnhookWindowsHookEx(emu: &mut emu::Emu) {
    let hhk = emu.regs().rcx;
    log_red!(emu, "user32!UnhookWindowsHookEx hhk:0x{:x}", hhk);
    if hhk != 0 {
        helper::handler_close(hhk);
    }
    emu.regs_mut().rax = 1;
}
