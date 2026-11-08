use crate::emu;
use crate::windows::constants;

pub fn timeGetTime(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!timeGetTime");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
