use crate::emu;

pub fn BaseWriteErrorElevationRequiredEvent(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BaseWriteErrorElevationRequiredEvent");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
