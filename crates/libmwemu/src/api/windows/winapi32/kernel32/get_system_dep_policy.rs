use crate::emu;
use crate::windows::constants;

pub fn GetSystemDEPPolicy(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDEPPolicy");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
