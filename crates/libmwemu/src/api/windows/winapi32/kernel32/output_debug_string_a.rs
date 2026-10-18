use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    let msg = emu.maps.read_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!OutputDebugStringA `{}`", msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
