use super::create_window;
use crate::emu;

pub(super) fn CreateWindowExA(emu: &mut emu::Emu) {
    create_window(emu, "CreateWindowExA", false);
}
