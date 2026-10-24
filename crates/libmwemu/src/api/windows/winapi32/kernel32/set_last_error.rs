use crate::emu;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!SetLastError err: {}", err_code);

    emu.set_last_error(err_code as u64);

    emu.stack_pop32(false);
}
