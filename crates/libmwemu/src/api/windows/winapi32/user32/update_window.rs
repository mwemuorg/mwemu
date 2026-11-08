use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn UpdateWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!UpdateWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
