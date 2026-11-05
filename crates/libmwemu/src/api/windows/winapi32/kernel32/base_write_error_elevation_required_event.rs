use crate::emu;
use crate::winapi::helper;

pub fn BaseWriteErrorElevationRequiredEvent(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BaseWriteErrorElevationRequiredEvent");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
}
