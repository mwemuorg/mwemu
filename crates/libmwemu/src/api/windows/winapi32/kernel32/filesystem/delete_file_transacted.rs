use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn DeleteFileTransactedA(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_string(str_ptr);
    log_red!(emu, "kernel32!DeleteFileTransactedA '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn DeleteFileTransactedW(emu: &mut emu::Emu) {
    let lpFileName = WinApiAbi::arg(emu, 0) as u32;
    let _hTransaction = WinApiAbi::arg(emu, 1) as u32;

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!DeleteFileTransactedW {}", lpFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
