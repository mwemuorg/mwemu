use crate::emu;
use crate::winapi::helper;

pub fn freopen(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(emu, "msvcrt!freopen '{}' => 0x{:x}", path, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
