use crate::emu;
use crate::windows::constants::errors::*;

pub fn DisableThreadProfiling(emu: &mut emu::Emu) {
    let _PerformanceDataHandle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!DisableThreadProfiling");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
