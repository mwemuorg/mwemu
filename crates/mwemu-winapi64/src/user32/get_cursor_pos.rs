use super::write_cursor_pos;
use crate::emu;

pub(super) fn GetCursorPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetCursorPos");
    write_cursor_pos(emu);
}
