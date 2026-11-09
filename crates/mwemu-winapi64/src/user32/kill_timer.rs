use crate::emu;
use crate::winapi::helper;

pub fn KillTimer(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let timer_id = emu.regs().rdx;
    helper::handler_close(timer_id);
    log_red!(emu, "user32!KillTimer hwnd:0x{:x} timer:{}", hwnd, timer_id);
    emu.regs_mut().rax = 1;
}
