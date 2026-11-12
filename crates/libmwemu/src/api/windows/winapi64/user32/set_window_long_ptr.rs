use super::get_window_long_value;
use crate::emu;

pub(super) fn SetWindowLongPtrA(emu: &mut emu::Emu) {
    set_window_long_ptr(emu, "SetWindowLongPtrA", false);
}

pub(super) fn SetWindowLongPtrW(emu: &mut emu::Emu) {
    set_window_long_ptr(emu, "SetWindowLongPtrW", true);
}

fn set_window_long_ptr(emu: &mut emu::Emu, api: &str, wide: bool) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(emu, "user32!{api} idx:{} new:0x{:x}", idx, new_val);
    if wide {
        emu.set_last_error(0);
    }

    emu.regs_mut().rax = get_window_long_value(idx);
}
