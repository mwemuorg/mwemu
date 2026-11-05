use crate::emu;
use crate::winapi::helper;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu.regs().rcx as u32;
    let _bManualReset = emu.regs().rdx as u32;
    let lpTimerName = emu.regs().r8 as u32;

    let lpTimerName = emu.maps.read_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerA {}", lpTimerName);

    emu.regs_mut().rax = helper::handler_create("CreateWaitableTimerA");
}
