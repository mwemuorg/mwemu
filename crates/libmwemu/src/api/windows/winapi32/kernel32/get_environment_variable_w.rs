use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetEnvironmentVariableW(emu: &mut emu::Emu) {
    let name = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!GetEnvironmentVariableW `{}` (not set)", name);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
