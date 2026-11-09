use crate::emu;
use crate::winapi::helper;

pub fn DrawMenuBar(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!DrawMenuBar hwnd:0x{:x}", hwnd);
    let result = if helper::handler_exist(hwnd) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
