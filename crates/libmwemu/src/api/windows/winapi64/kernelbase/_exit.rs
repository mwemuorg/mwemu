use crate::emu;

pub(super) fn _exit(emu: &mut emu::Emu) {
    let status = emu.regs().rcx;
    log_red!(emu, "kernelbase!_exit {}", status);
    emu.stop();
}
