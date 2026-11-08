use crate::emu;

pub fn GetCurrentThreadId(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentThreadId");

    emu.regs_mut().rax = 0x111; //TODO: track pids and tids
}
