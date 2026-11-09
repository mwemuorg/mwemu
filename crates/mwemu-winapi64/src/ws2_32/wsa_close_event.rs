use crate::emu;
use crate::winapi::helper;

pub fn WSACloseEvent(emu: &mut emu::Emu) {
    let event = emu.regs().rcx;
    log_red!(emu, "ws2_32!WSACloseEvent event: 0x{:x}", event);
    if event != 0 {
        helper::handler_close(event);
    }
    emu.regs_mut().rax = 1; // TRUE
}
