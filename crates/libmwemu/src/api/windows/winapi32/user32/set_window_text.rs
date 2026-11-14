use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SetWindowTextA(emu: &mut emu::Emu) {
    set_window_text(emu, "SetWindowTextA", false);
}

fn set_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let text = strings::read_text(emu, arg(emu, 1) as u64, wide);
    log_red!(emu, "user32!{api} '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub(super) fn SetWindowTextW(emu: &mut emu::Emu) {
    set_window_text(emu, "SetWindowTextW", true);
}
