use crate::emu;

pub fn IsDebuggerPresent(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsDebuggerPresent => FALSE");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
