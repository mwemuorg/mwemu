use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn CreateSemaphoreA(emu: &mut emu::Emu) {
    let _lpSemaphoreAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lInitialCount = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lMaximumCount = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

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
