use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!SetLastError err: {}", err_code);

    emu.set_last_error(err_code as u64);

    emu.stack_pop32(false);
}
