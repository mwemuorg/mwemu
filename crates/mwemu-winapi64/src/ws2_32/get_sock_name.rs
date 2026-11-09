use crate::emu;
use crate::winapi::helper;

pub(super) fn GetSockName(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let sockaddr_ptr = emu.regs().rdx;
    let namelen_ptr = emu.regs().r8;

    log_red!(emu, "ws2_32!GetSockName sock: 0x{:x}", sock);

    if !helper::socket_exist(sock) {
        emu.regs_mut().rax = !0u64; // SOCKET_ERROR
        return;
    }

    if sockaddr_ptr != 0 {
        emu.maps.write_word(sockaddr_ptr, 2); // AF_INET
        emu.maps.write_word(sockaddr_ptr + 2, 0); // port 0
        emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f); // 127.0.0.1
    }
    if namelen_ptr != 0 {
        emu.maps.write_dword(namelen_ptr, 16); // sizeof(sockaddr_in)
    }
    emu.regs_mut().rax = 0;
}
