use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    let _lpVersionInformation = WinApiAbi::arg(emu, 0) as u32;
    let _dwTypeMask = WinApiAbi::arg(emu, 1) as u32;
    let _dwlConditionMask = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!VerifyVersionInfoA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    stdcall_ret(emu, 4);
}

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!VerifyVersionInfoW");

    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 0xffff;
}
