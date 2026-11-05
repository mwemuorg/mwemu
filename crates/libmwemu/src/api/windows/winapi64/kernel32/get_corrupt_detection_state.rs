use crate::emu;

pub fn GetCorruptDetectionState(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCorruptDetectionState => 0");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
