use crate::emu;

pub fn __misaligned_access(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!__misaligned_access");
    emu.set_last_error(0);
}
