use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn ReplaceFile(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
    let lpReplacedFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpReplacementFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpBackupFileName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwReplaceFlags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpExclude = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _lpReserved = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

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
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!ReplaceFileW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 6);
    emu.regs_mut().rax = 1;
}
