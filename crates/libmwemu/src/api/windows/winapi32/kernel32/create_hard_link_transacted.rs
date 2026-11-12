use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn CreateHardLinkTransactedA(emu: &mut emu::Emu) {
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpExistingFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpSecurityAttributes = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpFileName = emu.maps.read_string(lpFileName as u64);
    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateHardLinkTransactedA {} {}",
        lpFileName,
        lpExistingFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn CreateHardLinkTransactedW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!CreateHardLinkTransactedW '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}
