use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SetFocus(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "user32!SetFocus hwnd:0x{:x}", hwnd);
    emu.regs_mut().rax = hwnd as u64;
    stdcall_ret(emu, 1);
}
