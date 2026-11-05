use crate::emu;

pub fn DebugBreak(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!DebugBreak triggered");
    emu.set_last_error(0);
}
