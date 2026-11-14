use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateSemaphoreA(emu: &mut emu::Emu) {
    let _lpSemaphoreAttributes = WinApiAbi::arg(emu, 0) as u32;
    let _lInitialCount = WinApiAbi::arg(emu, 1) as u32;
    let _lMaximumCount = WinApiAbi::arg(emu, 2) as u32;
    let lpName = WinApiAbi::arg(emu, 3) as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateSemaphoreA {}", lpName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn CreateSemaphoreW(emu: &mut emu::Emu) {
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateSemaphoreW");
    emu.regs_mut().rax = helper::handler_create("CreateSemaphoreW");
}
