use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;
use crate::windows::window_table;

pub(super) fn DestroyWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!DestroyWindow hwnd:0x{:x}", hwnd);
    if helper::handler_exist(hwnd) {
        helper::handler_close(hwnd);
    }
    emu.regs_mut().rax = 1;
    if window_table::destroy_window(hwnd) {
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        emu.regs_mut().rax = 0;
    }
}
