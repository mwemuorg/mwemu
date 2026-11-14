use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WSASetLastError(emu: &mut emu::Emu) {
    let err = arg(emu, 0);
    log_red!(emu, "ws2_32!WSASetLastError err: {}", err);
    emu.set_last_error(err as u64);
    stdcall_ret(emu, 1);
}
