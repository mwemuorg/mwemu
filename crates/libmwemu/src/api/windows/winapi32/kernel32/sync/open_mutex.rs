use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    let _dwDesiredAccess = WinApiAbi::arg(emu, 0) as u32;
    let _bInheritHandle = WinApiAbi::arg(emu, 1) as u32;
    let lpName = WinApiAbi::arg(emu, 2) as u32;

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
