use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SetWindowLongA(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongA");
}

fn set_window_long(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api} idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

pub(super) fn SetWindowLongW(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongW");
}
