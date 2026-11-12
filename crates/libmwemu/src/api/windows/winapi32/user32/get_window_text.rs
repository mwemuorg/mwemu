use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetWindowTextA(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextA");
}

fn get_window_text(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "user32!{api}");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

pub(super) fn GetWindowTextW(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextW");
}
