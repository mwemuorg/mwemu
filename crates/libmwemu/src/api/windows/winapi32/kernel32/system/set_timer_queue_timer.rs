use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetTimerQueueTimer(emu: &mut emu::Emu) {
    let _TimerQueue = WinApiAbi::arg(emu, 0) as u32;
    let _Callback = WinApiAbi::arg(emu, 1) as u32;
    let _Parameter = WinApiAbi::arg(emu, 2) as u32;
    let _DueTime = WinApiAbi::arg(emu, 3) as u32;
    let _Period = WinApiAbi::arg(emu, 4) as u32;
    let _PreferIo = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!SetTimerQueueTimer");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
