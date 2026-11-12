use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn DrawTextA(emu: &mut emu::Emu) {
    draw_text(emu, "DrawTextA", false);
}

fn draw_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let text = strings::read_text(emu, arg(emu, 1) as u64, wide);
    log_red!(emu, "user32!{api} '{}'", text);
    emu.regs_mut().rax = 20;
    stdcall_ret(emu, 5);
}

pub(super) fn DrawTextW(emu: &mut emu::Emu) {
    draw_text(emu, "DrawTextW", true);
}
