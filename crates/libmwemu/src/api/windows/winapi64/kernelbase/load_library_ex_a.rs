use crate::emu;
use crate::winapi::helper;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let name = emu.maps.read_string(name_ptr);
    let hndl = helper::handler_create(&format!("module_{}", name));
    log_red!(emu, "kernelbase!LoadLibraryExA '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
