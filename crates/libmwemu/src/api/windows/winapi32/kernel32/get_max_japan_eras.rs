use crate::emu;
use crate::winapi::helper;

pub fn GetMaxJapanEras(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaxJapanEras");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
