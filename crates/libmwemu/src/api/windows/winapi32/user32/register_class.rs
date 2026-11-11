use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn RegisterClassA(emu: &mut emu::Emu) {
    let wndclass = arg(emu, 0);
    log_red!(emu, "user32!RegisterClassA 0x{:x}", wndclass);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

pub(super) fn RegisterClassW(emu: &mut emu::Emu) {
    let wndclass = arg(emu, 0);
    log_red!(emu, "user32!RegisterClassW 0x{:x}", wndclass);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
