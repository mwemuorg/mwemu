use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let hdc = arg(emu, 1);
    log_red!(emu, "user32!ReleaseDC hwnd: 0x{:x} hdc: 0x{:x}", hwnd, hdc);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
