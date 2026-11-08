use crate::emu;
use crate::windows::constants;

pub fn IsCallerAdminOrSystem(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsCallerAdminOrSystem");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
