use super::get_window_long_value;
use crate::emu;

pub(super) fn GetWindowLongA(emu: &mut emu::Emu) {
    get_window_long(emu, "GetWindowLongA");
}

fn get_window_long(emu: &mut emu::Emu, api: &str) {
    let idx = emu.regs().rdx as i32;
    let result = get_window_long_value(idx);
    log_red!(emu, "user32!{api} idx:{} => 0x{:x}", idx, result);
    emu.regs_mut().rax = result;
}

pub(super) fn GetWindowLongW(emu: &mut emu::Emu) {
    get_window_long(emu, "GetWindowLongW");
}
