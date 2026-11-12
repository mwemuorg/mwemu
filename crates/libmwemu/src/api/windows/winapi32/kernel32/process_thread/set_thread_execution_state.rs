use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetThreadExecutionState(emu: &mut emu::Emu) {
    let _esFlags = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!SetThreadExecutionState");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
