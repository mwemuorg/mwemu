use super::create_window;
use crate::emu;

pub(super) fn CreateWindowExW(emu: &mut emu::Emu) {
    create_window(emu, "CreateWindowExW", true);
}
