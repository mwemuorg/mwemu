use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi64::kernel32::load_library;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    load_library_ex(emu, "LoadLibraryExA", false);
}

fn load_library_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let dllptr = emu.regs().rcx;
    let dll = strings::read_text(emu, dllptr, wide);

    emu.set_last_error(0);
    emu.regs_mut().rax = load_library(emu, &dll);

    log_red!(emu, "kernel32!{api}  '{}' =0x{:x}", dll, emu.regs().rax);
}

pub fn LoadLibraryExW(emu: &mut emu::Emu) {
    load_library_ex(emu, "LoadLibraryExW", true);
}
