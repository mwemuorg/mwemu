use crate::emu;
use crate::winapi::helper;

pub fn SetEvent(emu: &mut emu::Emu) {
    let hEvent = emu.regs().rcx;
    log_red!(emu, "kernelbase!SetEvent 0x{:x}", hEvent);
    if helper::handler_exist(hEvent) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
    }
}
