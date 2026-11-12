use crate::emu;
use crate::winapi::helper;

pub fn OleUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!OleUninitialize");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
}
