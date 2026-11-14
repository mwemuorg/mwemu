use crate::emu;
use crate::winapi::common::strings;

pub(super) fn SetWindowTextA(emu: &mut emu::Emu) {
    set_window_text(emu, "SetWindowTextA", false);
}

fn set_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let text = strings::read_text(emu, emu.regs().rdx, wide);
    log_red!(emu, "user32!{api} '{}'", text);
    emu.regs_mut().rax = 1;
}

pub(super) fn SetWindowTextW(emu: &mut emu::Emu) {
    set_window_text(emu, "SetWindowTextW", true);
}
