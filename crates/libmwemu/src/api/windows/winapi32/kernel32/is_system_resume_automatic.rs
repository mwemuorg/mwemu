use crate::emu;
use crate::windows::constants;

pub fn IsSystemResumeAutomatic(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsSystemResumeAutomatic");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
