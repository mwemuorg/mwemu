use crate::emu;
use crate::winapi::helper;

pub fn WSAAsyncSelect(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hwnd = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let msg = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let events = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
