use crate::emu;

pub(super) fn exit(emu: &mut emu::Emu) {
    let status = emu.regs().rcx;
    log_red!(emu, "kernelbase!exit {}", status);
    emu.stop();
}
