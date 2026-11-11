use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SetWindowLongA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowLongA idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

pub(super) fn SetWindowLongW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowLongW idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
