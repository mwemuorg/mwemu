use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetMaxJapanEras(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaxJapanEras");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
