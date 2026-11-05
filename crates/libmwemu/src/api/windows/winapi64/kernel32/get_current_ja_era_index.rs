use crate::emu;

pub fn GetCurrentJaEraIndex(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentJaEraIndex => 4 (Reiwa)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 4;
}
