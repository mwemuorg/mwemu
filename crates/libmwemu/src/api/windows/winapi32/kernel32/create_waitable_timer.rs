use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerA", false);
}

fn create_waitable_timer(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _lpTimerAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _bManualReset = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpTimerName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
