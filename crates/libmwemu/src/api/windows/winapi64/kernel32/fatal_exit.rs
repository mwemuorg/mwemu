use crate::emu;

pub fn FatalExit(emu: &mut emu::Emu) {
    let exit_code = emu.regs().rcx;
    log_red!(emu, "kernel32!FatalExit exitCode:{}", exit_code);
    emu.running = false;
    emu.set_last_error(0);
}
