use crate::emu;
use crate::windows::constants;

pub fn BasepGetMiniVersionForCreate(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepGetMiniVersionForCreate");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
