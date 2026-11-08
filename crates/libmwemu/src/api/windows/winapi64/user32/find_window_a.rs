use super::find_window;
use crate::emu;

pub fn FindWindowA(emu: &mut emu::Emu) {
    let class_ptr = emu.regs().rcx;
    let window_ptr = emu.regs().rdx;
    find_window(emu, "FindWindowA", 0, 0, class_ptr, window_ptr, false);
}
