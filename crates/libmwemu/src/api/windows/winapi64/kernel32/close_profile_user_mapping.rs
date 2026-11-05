use crate::emu;

pub fn CloseProfileUserMapping(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CloseProfileUserMapping");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
