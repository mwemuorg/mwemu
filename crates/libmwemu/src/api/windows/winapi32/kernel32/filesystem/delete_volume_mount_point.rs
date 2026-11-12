use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn DeleteVolumeMountPointA(emu: &mut emu::Emu) {
    let _lpszVolumeMountPoint = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!DeleteVolumeMountPointA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn DeleteVolumeMountPointW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!DeleteVolumeMountPointW '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 1);
    emu.regs_mut().rax = 1;
}
