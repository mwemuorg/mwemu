use crate::emu;
use crate::winapi::common::strings;

pub(super) fn DrawTextA(emu: &mut emu::Emu) {
    draw_text(emu, "DrawTextA", false);
}

fn draw_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let text = strings::read_text(emu, emu.regs().rdx, wide);
    log_red!(emu, "user32!{api} '{}'", text);
    emu.regs_mut().rax = 20;
}

pub(super) fn DrawTextW(emu: &mut emu::Emu) {
    draw_text(emu, "DrawTextW", true);
}
