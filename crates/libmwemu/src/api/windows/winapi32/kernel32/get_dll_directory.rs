use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetDllDirectoryA(emu: &mut emu::Emu) {
    get_dll_directory(emu, "GetDllDirectoryA");
}

fn get_dll_directory(emu: &mut emu::Emu, api: &str) {
    let _nBufferLength = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpBuffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn GetDllDirectoryW(emu: &mut emu::Emu) {
    get_dll_directory(emu, "GetDllDirectoryW");
}
