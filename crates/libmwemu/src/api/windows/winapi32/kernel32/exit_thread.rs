use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ExitThread(emu: &mut emu::Emu) {
    let code = arg(emu, 0);
    log_red!(emu, "kernel32!ExitThread code: {}", code);
    emu.stop();
    stdcall_ret(emu, 1);
}
