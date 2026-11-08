use crate::emu;
use crate::windows::constants;

pub fn OpenSortIdKey(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSortIdKey");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
