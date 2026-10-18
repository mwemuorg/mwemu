use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    let msg = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!OutputDebugStringW `{}`", msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
