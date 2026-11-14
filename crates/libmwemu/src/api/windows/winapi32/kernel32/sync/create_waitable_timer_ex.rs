use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateWaitableTimerExA(emu: &mut emu::Emu) {
    let _lpTimerAttributes = WinApiAbi::arg(emu, 0) as u32;
    let lpTimerName = WinApiAbi::arg(emu, 1) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 2) as u32;
    let _dwDesiredAccess = WinApiAbi::arg(emu, 3) as u32;

    let lpTimerName = emu.maps.read_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerExA {}", lpTimerName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn CreateWaitableTimerExW(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateWaitableTimerExW");
    emu.regs_mut().rax = helper::handler_create("CreateWaitableTimerExW");
}
