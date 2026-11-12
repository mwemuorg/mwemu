use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn ReplaceFile(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "kernel32!ReplaceFile h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn ReplaceFileA(emu: &mut emu::Emu) {
    let lpReplacedFileName = WinApiAbi::arg(emu, 0) as u32;
    let lpReplacementFileName = WinApiAbi::arg(emu, 1) as u32;
    let lpBackupFileName = WinApiAbi::arg(emu, 2) as u32;
    let _dwReplaceFlags = WinApiAbi::arg(emu, 3) as u32;
    let _lpExclude = WinApiAbi::arg(emu, 4) as u32;
    let _lpReserved = WinApiAbi::arg(emu, 5) as u32;

    let lpReplacedFileName = emu.maps.read_string(lpReplacedFileName as u64);
    let lpReplacementFileName = emu.maps.read_string(lpReplacementFileName as u64);
    let lpBackupFileName = emu.maps.read_string(lpBackupFileName as u64);

    log_red!(
        emu,
        "kernel32!ReplaceFileA {} {} {}",
        lpReplacedFileName,
        lpReplacementFileName,
        lpBackupFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn ReplaceFileW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!ReplaceFileW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 6);
    emu.regs_mut().rax = 1;
}
