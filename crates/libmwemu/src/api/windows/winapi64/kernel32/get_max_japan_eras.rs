use crate::emu;

pub fn GetMaxJapanEras(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaxJapanEras => 5");
    emu.set_last_error(0);
    emu.regs_mut().rax = 5;
}
