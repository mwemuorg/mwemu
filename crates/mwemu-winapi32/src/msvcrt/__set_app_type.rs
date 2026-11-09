use crate::emu;

pub(super) fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
    emu.set_last_error(0);
}
