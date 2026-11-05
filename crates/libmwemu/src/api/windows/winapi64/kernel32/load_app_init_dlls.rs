use crate::emu;

pub fn LoadAppInitDlls(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LoadAppInitDlls");
    emu.set_last_error(0);
}
