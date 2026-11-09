use crate::emu;
use crate::winapi::helper;

pub fn WSAAsyncSelect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let hwnd = emu.regs().rdx;
    let msg = emu.regs().r8;
    let events = emu.regs().r9;
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
}
