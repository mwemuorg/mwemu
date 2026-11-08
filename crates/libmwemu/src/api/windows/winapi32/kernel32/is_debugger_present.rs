use crate::emu;

pub fn IsDebuggerPresent(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsDebuggerPresent");
    emu.regs_mut().rax = 0; // of course :p
}
