use crate::emu;
use crate::windows::constants;

pub fn CptpQuirksInitOnce(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CptpQuirksInitOnce");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
