use super::get_window_long_value;
use crate::emu;

pub(super) fn SetWindowLongA(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongA");
}

fn set_window_long(emu: &mut emu::Emu, api: &str) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(emu, "user32!{api} idx:{} new:0x{:x}", idx, new_val);
    emu.regs_mut().rax = get_window_long_value(idx);
}

pub(super) fn SetWindowLongW(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongW");
}
