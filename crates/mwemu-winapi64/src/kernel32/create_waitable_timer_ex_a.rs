use crate::emu;
use crate::winapi::helper;

pub fn CreateWaitableTimerExA(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu.regs().rcx as u32;
    let lpTimerName = emu.regs().rdx as u32;
    let _dwFlags = emu.regs().r8 as u32;
    let _dwDesiredAccess = emu.regs().r9 as u32;

    let lpTimerName = emu.maps.read_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerExA {}", lpTimerName);

    emu.regs_mut().rax = helper::handler_create("CreateWaitableTimerExA");
}
