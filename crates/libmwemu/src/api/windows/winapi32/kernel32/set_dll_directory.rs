use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn SetDllDirectoryA(emu: &mut emu::Emu) {
    set_dll_directory(emu, "SetDllDirectoryA", false);
}

fn set_dll_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpPathName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpPathName = strings::read_text(emu, lpPathName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpPathName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn SetDllDirectoryW(emu: &mut emu::Emu) {
    set_dll_directory(emu, "SetDllDirectoryW", true);
}
