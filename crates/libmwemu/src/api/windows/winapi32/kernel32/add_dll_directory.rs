use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn AddDllDirectory(emu: &mut emu::Emu) {
    let new_directory = arg(emu, 0) as u64;
    let dir = emu.maps.read_wide_string(new_directory);
    log_red!(emu, "kernel32!AddDllDirectory '{}'", dir);
    let h = helper::handler_create(&format!("dlldir://{}", dir));
    emu.regs_mut().rax = h;
    stdcall_ret(emu, 1);
}
