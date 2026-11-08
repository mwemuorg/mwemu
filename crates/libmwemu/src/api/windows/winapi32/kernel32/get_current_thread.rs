use crate::emu;
use crate::windows::constants;

pub fn GetCurrentThread(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentThread");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
