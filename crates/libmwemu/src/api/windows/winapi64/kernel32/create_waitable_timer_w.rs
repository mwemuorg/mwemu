use crate::emu;
use crate::winapi::helper;

pub fn CreateWaitableTimerW(emu: &mut emu::Emu) {
    let _lpTimerAttributes = emu.regs().rcx as u32;
    let _bManualReset = emu.regs().rdx as u32;
    let lpTimerName = emu.regs().r8 as u32;

    let lpTimerName = emu.maps.read_wide_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerW {}", lpTimerName);

    emu.regs_mut().rax = helper::handler_create("CreateWaitableTimerW");
}
