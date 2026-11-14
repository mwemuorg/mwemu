use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn PostQuitMessage(emu: &mut emu::Emu) {
    let exit_code = arg(emu, 0) as i32;
    log_red!(emu, "user32!PostQuitMessage code:{}", exit_code);
    stdcall_ret(emu, 1);
}
