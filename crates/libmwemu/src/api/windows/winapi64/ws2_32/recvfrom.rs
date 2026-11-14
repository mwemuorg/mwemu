use crate::emu;
use crate::winapi::helper;

pub(super) fn recvfrom(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buf = emu.regs().rdx;
    let _len = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    log_red!(emu, "ws2_32!recvfrom sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}
