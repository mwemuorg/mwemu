use crate::emu;
use crate::winapi::winapi32::kernel32::LAST_ERROR;

pub fn SetLastError(emu: &mut emu::Emu) {
    let err_code = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!SetLastError cannot read err_code");

    log_red!(emu, "kernel32!SetLastError err: {}", err_code);

    let mut err = LAST_ERROR.lock().unwrap();
    *err = err_code;

    emu.stack_pop32(false);
}
