use crate::emu;
use crate::windows::constants;

pub fn IsSystemLUID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsSystemLUID");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
