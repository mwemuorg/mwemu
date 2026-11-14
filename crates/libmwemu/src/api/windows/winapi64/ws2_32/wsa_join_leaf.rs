use crate::emu;
use crate::winapi::helper;

pub fn WSAJoinLeaf(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let namelen = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAJoinLeaf sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = sock;
    }
}
