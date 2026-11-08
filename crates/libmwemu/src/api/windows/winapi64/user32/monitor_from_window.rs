use crate::emu;
use crate::winapi::helper;

pub fn MonitorFromWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let hmon = helper::handler_create("monitor");
    log_red!(
        emu,
        "user32!MonitorFromWindow hwnd:0x{:x} flags:0x{:x} => 0x{:x}",
        hwnd,
        flags,
        hmon
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hmon;
}
