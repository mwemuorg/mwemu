use super::write_cursor_pos;
use crate::emu;

/// Same point as GetCursorPos: the emulated desktop has no DPI scaling.
pub(super) fn GetPhysicalCursorPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetPhysicalCursorPos");
    write_cursor_pos(emu);
}
