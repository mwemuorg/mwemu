use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerA", false);
}

fn create_waitable_timer(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _lpTimerAttributes = WinApiAbi::arg(emu, 0) as u32;
    let _bManualReset = WinApiAbi::arg(emu, 1) as u32;
    let lpTimerName = WinApiAbi::arg(emu, 2) as u32;

    let lpTimerName = strings::read_text(emu, lpTimerName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpTimerName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn CreateWaitableTimerW(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerW", true);
}
