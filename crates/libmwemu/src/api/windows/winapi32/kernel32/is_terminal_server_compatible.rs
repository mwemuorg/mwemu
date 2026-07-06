use crate::emu;
use crate::windows::constants;

pub fn IsTerminalServerCompatible(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsTerminalServerCompatible");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
