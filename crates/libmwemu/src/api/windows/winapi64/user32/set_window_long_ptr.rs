use super::get_window_long_value;
use crate::emu;

pub(super) fn SetWindowLongPtrA(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(
        emu,
        "user32!SetWindowLongPtrA idx:{} new:0x{:x}",
        idx,
        new_val
    );
    emu.regs_mut().rax = get_window_long_value(idx);
}

pub(super) fn SetWindowLongPtrW(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(
        emu,
        "user32!SetWindowLongPtrW idx:{} new:0x{:x}",
        idx,
        new_val
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = get_window_long_value(idx);
}
