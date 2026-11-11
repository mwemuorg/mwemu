use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    let _dwDesiredAccess = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _bInheritHandle = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!OpenMutexA {}", lpName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn OpenMutexW(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!OpenMutexW");
    emu.regs_mut().rax = helper::handler_create("OpenMutexW");
}
