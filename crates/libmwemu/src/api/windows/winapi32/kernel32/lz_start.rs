use crate::emu;
use crate::windows::constants;

pub fn LZStart(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LZStart");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
