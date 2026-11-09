use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    let msg = emu.maps.read_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!OutputDebugStringA `{}`", msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    let msg = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!OutputDebugStringW `{}`", msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
