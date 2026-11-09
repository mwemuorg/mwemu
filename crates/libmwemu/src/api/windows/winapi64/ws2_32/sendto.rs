use crate::emu;
use crate::winapi::helper;

pub(super) fn sendto(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let _buf = emu.regs().rdx;
    let len = emu.regs().r8;
    let _flags = emu.regs().r9 as u32;
    log_red!(emu, "ws2_32!sendto sock:0x{:x} len:{}", sock, len);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    } else {
        emu.regs_mut().rax = len;
    }
}
