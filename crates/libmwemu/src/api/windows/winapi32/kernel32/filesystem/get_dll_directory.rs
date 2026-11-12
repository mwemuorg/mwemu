use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetDllDirectoryA(emu: &mut emu::Emu) {
    get_dll_directory(emu, "GetDllDirectoryA");
}

fn get_dll_directory(emu: &mut emu::Emu, api: &str) {
    let _nBufferLength = WinApiAbi::arg(emu, 0) as u32;
    let _lpBuffer = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn GetDllDirectoryW(emu: &mut emu::Emu) {
    get_dll_directory(emu, "GetDllDirectoryW");
}
