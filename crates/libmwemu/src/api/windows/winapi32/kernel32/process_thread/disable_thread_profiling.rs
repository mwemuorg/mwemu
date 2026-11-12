use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn DisableThreadProfiling(emu: &mut emu::Emu) {
    let _PerformanceDataHandle = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!DisableThreadProfiling");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
