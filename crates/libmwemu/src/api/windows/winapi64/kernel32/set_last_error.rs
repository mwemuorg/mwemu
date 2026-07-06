use crate::emu;
use crate::winapi::winapi64::kernel32::set_last_error;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = emu.regs().rcx;

    log_red!(emu, "kernel32!SetLastError err: {}", err_code);
    set_last_error(err_code);
}
