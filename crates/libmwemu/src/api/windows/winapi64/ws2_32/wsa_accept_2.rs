use crate::emu;
use crate::winapi::helper;

pub(super) fn WsaAccept(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let saddr = emu.regs().rdx;
    let addrlen = emu.regs().r8;

    log_red!(emu, "ws2_32!WsaAccept  sock: 0x{:x}", sock);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = !0u64; // INVALID_SOCKET
    } else {
        let new_sock = helper::socket_create();
        if saddr != 0 && addrlen != 0 {
            emu.maps.write_word(saddr, 2); // AF_INET
            emu.maps.write_word(saddr + 2, 0x5000); // port 80 network order
            emu.maps.write_dword(saddr + 4, 0x0100007f); // 127.0.0.1
        }
        emu.regs_mut().rax = new_sock;
    }
}
