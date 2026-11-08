use crate::emu;
use crate::windows::constants;

pub fn _guard_check_icall_nop(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_guard_check_icall_nop");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
