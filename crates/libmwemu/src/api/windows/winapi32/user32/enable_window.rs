use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn EnableWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EnableWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
