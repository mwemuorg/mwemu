use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreA(emu: &mut emu::Emu) {
    let _lpSemaphoreAttributes = emu.regs().rcx as u32;
    let _lInitialCount = emu.regs().rdx as u32;
    let _lMaximumCount = emu.regs().r8 as u32;
    let lpName = emu.regs().r9 as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateSemaphoreA {}", lpName);

    emu.regs_mut().rax = helper::handler_create("CreateSemaphoreA");
}
