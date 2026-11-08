use super::find_window;
use crate::emu;

pub fn FindWindowW(emu: &mut emu::Emu) {
    let class_ptr = emu.regs().rcx;
    let window_ptr = emu.regs().rdx;
    find_window(emu, "FindWindowW", 0, 0, class_ptr, window_ptr, true);
}
