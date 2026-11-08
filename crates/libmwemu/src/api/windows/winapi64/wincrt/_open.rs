use crate::emu;
use crate::winapi::helper;

pub fn _open(emu: &mut emu::Emu, api: &str) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(emu, "wincrt!{} '{}' => 0x{:x}", api, path, hndl);
    emu.regs_mut().rax = hndl;
}
