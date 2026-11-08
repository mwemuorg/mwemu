use crate::emu;
use crate::winapi::helper;

pub fn WSAResetEvent(emu: &mut emu::Emu) {
    let hEvent = emu.regs().rcx;
    log_red!(emu, "ws2_32!WSAResetEvent 0x{:x}", hEvent);
    if helper::handler_exist(hEvent) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
    }
}
