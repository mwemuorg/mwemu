use crate::emu;
use crate::winapi::helper;

pub fn ReleaseMutex(emu: &mut emu::Emu) {
    let h_mutex = emu.regs().rcx;

    log_red!(emu, "kernel32!ReleaseMutex hMutex:0x{:x}", h_mutex);

    if !helper::handler_exist(h_mutex) {
        log_red!(emu, "kernel32!ReleaseMutex invalid handle");
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
