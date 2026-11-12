use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn DefineDosDeviceA(emu: &mut emu::Emu) {
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpDeviceName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpTargetPath = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpDeviceName = emu.maps.read_string(lpDeviceName as u64);
    let lpTargetPath = emu.maps.read_string(lpTargetPath as u64);

    log_red!(
        emu,
        "kernel32!DefineDosDeviceA {} {}",
        lpDeviceName,
        lpTargetPath
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn DefineDosDeviceW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!DefineDosDeviceW '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}
