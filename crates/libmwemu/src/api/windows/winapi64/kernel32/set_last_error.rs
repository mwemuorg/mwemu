use crate::emu;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = emu.regs().rcx;

    log_red!(emu, "kernel32!SetLastError err: {}", err_code);
    emu.set_last_error(err_code);
}
