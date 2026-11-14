use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    load_library(emu, "LoadLibraryA", false);
}

fn load_library(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let name = strings::read_text(emu, name_ptr, wide);
    let hndl = helper::handler_create(&format!("module_{}", name));
    log_red!(emu, "kernelbase!{api} '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    load_library(emu, "LoadLibraryW", true);
}
