use crate::emu;
use crate::windows::constants;

pub fn IsThreadAFiber(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsThreadAFiber");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
