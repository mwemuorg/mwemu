use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    load_library_ex(emu, "LoadLibraryExA", false);
}

fn load_library_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let name = strings::read_text(emu, name_ptr, wide);
    let hndl = helper::handler_create(&format!("module_{}", name));
    log_red!(emu, "kernelbase!{api} '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn LoadLibraryExW(emu: &mut emu::Emu) {
    load_library_ex(emu, "LoadLibraryExW", true);
}
