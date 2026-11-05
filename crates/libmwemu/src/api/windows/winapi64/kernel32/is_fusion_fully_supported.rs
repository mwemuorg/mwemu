use crate::emu;

pub fn IsFusionFullySupported(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsFusionFullySupported => TRUE");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
