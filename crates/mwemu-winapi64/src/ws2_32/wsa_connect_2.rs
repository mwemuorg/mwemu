use crate::emu;
use crate::winapi::helper;

pub(super) fn WsaConnect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _name = emu.regs().rdx;
    let _namelen = emu.regs().r8 as u32;
    log_red!(emu, "ws2_32!WSAConnect sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}
