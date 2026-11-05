use crate::emu;

pub fn GetMaxJapanYears(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaxJapanYears => 99");
    emu.set_last_error(0);
    emu.regs_mut().rax = 99;
}
