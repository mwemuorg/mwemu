use crate::emu;
use crate::winapi::helper;

pub fn WSAIoctl(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let ioctl_code = emu.regs().rdx;
    let in_buf_ptr = emu.regs().r8;
    let in_buf_len = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSAIoctl sock:0x{:x} code:0x{:x}",
        sock,
        ioctl_code
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}
