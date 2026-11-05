use crate::emu;

pub fn GetKnownJapanEraCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetKnownJapanEraCount => 5");
    emu.set_last_error(0);
    emu.regs_mut().rax = 5;
}
