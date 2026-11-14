use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSAAsyncSelect(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let hwnd = WinApiAbi::arg(emu, 1);
    let msg = WinApiAbi::arg(emu, 2) as u32;
    let events = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!WSAAsyncSelect sock:0x{:x} hwnd:0x{:x} msg:{} events:0x{:x}",
        sock,
        hwnd,
        msg,
        events
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
