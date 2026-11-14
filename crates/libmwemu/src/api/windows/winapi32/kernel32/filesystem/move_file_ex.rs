use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn MoveFileExA(emu: &mut emu::Emu) {
    let lpExistingFileName = WinApiAbi::arg(emu, 0) as u32;
    let lpNewFileName = WinApiAbi::arg(emu, 1) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 2) as u32;

    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!MoveFileExA {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn MoveFileExW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!MoveFileExW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
