use crate::emu;
use crate::windows::constants;

pub fn Wow64SystemServiceCall(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!Wow64SystemServiceCall");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
