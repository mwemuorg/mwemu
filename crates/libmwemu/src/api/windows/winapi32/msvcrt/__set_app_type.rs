use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
    emu.set_last_error(0);
}
