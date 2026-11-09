use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDirectoryA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!CreateDirectoryA `{}` (stub)", path);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn CreateDirectoryW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!CreateDirectoryW `{}` (stub)", path);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
