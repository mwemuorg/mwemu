use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetWindowLongA(emu: &mut emu::Emu) {
    get_window_long(emu, "GetWindowLongA");
}

fn get_window_long(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api} idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

pub(super) fn GetWindowLongW(emu: &mut emu::Emu) {
    get_window_long(emu, "GetWindowLongW");
}
