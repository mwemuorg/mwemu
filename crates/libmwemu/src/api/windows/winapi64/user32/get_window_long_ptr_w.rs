use super::get_window_long_value;
use crate::emu;

pub(super) fn GetWindowLongPtrW(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let result = get_window_long_value(idx);
    log_red!(
        emu,
        "user32!GetWindowLongPtrW idx:{} => 0x{:x}",
        idx,
        result
    );
    emu.regs_mut().rax = result;
}
