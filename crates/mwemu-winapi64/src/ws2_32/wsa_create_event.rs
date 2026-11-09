use crate::emu;
use crate::winapi::helper;

pub fn WSACreateEvent(emu: &mut emu::Emu) {
    let event = helper::handler_create("wsa_event");
    log_red!(emu, "ws2_32!WSACreateEvent -> 0x{:x}", event);
    emu.regs_mut().rax = event;
}
