use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn SetFileAttributesTransactedA(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_string(str_ptr);
    log_red!(
        emu,
        "kernel32!SetFileAttributesTransactedA '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}

pub fn SetFileAttributesTransactedW(emu: &mut emu::Emu) {
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwFileAttributes = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!SetFileAttributesTransactedW {}", lpFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
