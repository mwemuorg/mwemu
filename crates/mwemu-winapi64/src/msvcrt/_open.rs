use crate::emu;
use crate::winapi::helper;

pub fn _open(emu: &mut emu::Emu, api: &str) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, path);
    emu.regs_mut().rax = helper::handler_create(&path);
}
