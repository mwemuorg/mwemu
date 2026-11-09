use crate::emu;
use crate::windows::constants::errors::*;

pub fn SetDllDirectoryA(emu: &mut emu::Emu) {
    let lpPathName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpPathName = emu.maps.read_string(lpPathName as u64);

    log_red!(emu, "kernel32!SetDllDirectoryA {}", lpPathName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn SetDllDirectoryW(emu: &mut emu::Emu) {
    let lpPathName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    let lpPathName = emu.maps.read_wide_string(lpPathName as u64);

    log_red!(emu, "kernel32!SetDllDirectoryW {}", lpPathName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
