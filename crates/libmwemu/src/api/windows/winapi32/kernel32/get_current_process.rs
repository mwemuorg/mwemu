use crate::emu;
use crate::windows::constants;

pub fn GetCurrentProcess(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentProcess");
    emu.regs_mut().rax = constants::CURRENT_PROCESS_HANDLE;
}
