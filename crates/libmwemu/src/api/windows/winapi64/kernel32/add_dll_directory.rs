use crate::api::windows::helper;
use crate::emu;

pub fn AddDllDirectory(emu: &mut emu::Emu) {
    let new_directory = emu.regs().rcx;
    let dir = emu.maps.read_wide_string(new_directory);
    log_red!(emu, "kernel32!AddDllDirectory '{}'", dir);
    let h = helper::handler_create(&format!("dlldir://{}", dir));
    emu.regs_mut().rax = h;
}
