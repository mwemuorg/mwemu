use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    let _lpVersionInformation = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwTypeMask = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwlConditionMask = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!VerifyVersionInfoA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    stdcall_ret(emu, 4);
}

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!VerifyVersionInfoW");

    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 0xffff;
}
