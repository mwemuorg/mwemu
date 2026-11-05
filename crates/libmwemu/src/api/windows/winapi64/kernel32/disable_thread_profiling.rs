use crate::emu;

pub fn DisableThreadProfiling(emu: &mut emu::Emu) {
    let _PerformanceDataHandle = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!DisableThreadProfiling");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
