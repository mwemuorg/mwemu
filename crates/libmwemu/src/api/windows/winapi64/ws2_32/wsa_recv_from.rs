use crate::emu;
use crate::winapi::helper;

pub fn WSARecvFrom(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buffers_ptr = emu.regs().rdx;
    let buffer_count = emu.regs().r8 as u32;
    let bytes_recvd_ptr = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSARecvFrom sock:0x{:x} buffers:{}",
        sock,
        buffer_count
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if bytes_recvd_ptr != 0 {
            emu.maps.write_dword(bytes_recvd_ptr, 0);
        }
        emu.regs_mut().rax = 0;
    }
}
