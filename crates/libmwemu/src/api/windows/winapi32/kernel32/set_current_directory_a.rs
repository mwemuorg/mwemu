use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetCurrentDirectoryA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!SetCurrentDirectoryA `{}` (stub)", path);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
