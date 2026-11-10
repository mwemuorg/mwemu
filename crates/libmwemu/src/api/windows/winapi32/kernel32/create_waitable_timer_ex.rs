use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn CreateWaitableTimerExA(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpTimerName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwDesiredAccess = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

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
