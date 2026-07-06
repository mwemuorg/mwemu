use crate::emu;

pub fn ResetEvent(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!ResetEvent");
    // TODO: do something
    emu.regs_mut().rax = 1;
}
