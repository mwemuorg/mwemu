use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSAEventSelect(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let event = WinApiAbi::arg(emu, 1);
    let events = WinApiAbi::arg(emu, 2) as u32;
    log_red!(
        emu,
        "ws2_32!WSAEventSelect sock:0x{:x} event:0x{:x} events:0x{:x}",
        sock,
        event,
        events
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
