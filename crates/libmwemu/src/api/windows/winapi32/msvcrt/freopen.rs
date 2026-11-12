use crate::emu;
use crate::winapi::helper;

pub fn freopen(emu: &mut emu::Emu) {
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let path = emu.maps.read_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(emu, "msvcrt!freopen '{}' => 0x{:x}", path, hndl);
    emu.regs_mut().rax = hndl;
}
