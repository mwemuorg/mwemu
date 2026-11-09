use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let hdc = arg(emu, 1) as u64;
    log_red!(emu, "gdi32!ReleaseDC hwnd:0x{:x} hdc:0x{:x}", hwnd, hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
