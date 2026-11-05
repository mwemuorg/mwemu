use crate::emu;

pub fn LoadAppInitDllsImplementation(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LoadAppInitDllsImplementation");
    emu.set_last_error(0);
}
