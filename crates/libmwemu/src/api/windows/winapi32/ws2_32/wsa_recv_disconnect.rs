use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSARecvDisconnect(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let data_ptr = WinApiAbi::arg(emu, 1);
    log_red!(emu, "ws2_32!WSARecvDisconnect sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
