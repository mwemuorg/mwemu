use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn DeleteVolumeMountPointA(emu: &mut emu::Emu) {
    let _lpszVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!DeleteVolumeMountPointA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn DeleteVolumeMountPointW(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
