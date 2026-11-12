use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn CopyFileExA(emu: &mut emu::Emu) {
    let lpExistingFileName = WinApiAbi::arg(emu, 0) as u32;
    let lpNewFileName = WinApiAbi::arg(emu, 1) as u32;
    let _lpProgressRoutine = WinApiAbi::arg(emu, 2) as u32;
    let _lpData = WinApiAbi::arg(emu, 3) as u32;
    let _pbCancel = WinApiAbi::arg(emu, 4) as u32;
    let _dwCopyFlags = WinApiAbi::arg(emu, 5) as u32;

    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!CopyFileExA {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn CopyFileExW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!CopyFileExW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 6);
    emu.regs_mut().rax = 1;
}
