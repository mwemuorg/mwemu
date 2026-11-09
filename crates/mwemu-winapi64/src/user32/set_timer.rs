use crate::emu;
use crate::winapi::helper;

pub fn SetTimer(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let id = emu.regs().rdx;
    let elapse = emu.regs().r8 as u32;
    let callback = emu.regs().r9;
    let timer_handle = helper::handler_create("timer");
    log_red!(
        emu,
        "user32!SetTimer hwnd:0x{:x} id:{} ms:{} cb:0x{:x} => 0x{:x}",
        hwnd,
        id,
        elapse,
        callback,
        timer_handle
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = timer_handle;
}
