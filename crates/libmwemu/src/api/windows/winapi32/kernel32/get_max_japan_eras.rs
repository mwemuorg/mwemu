use crate::emu;
use crate::windows::constants;

pub fn GetMaxJapanEras(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaxJapanEras");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
