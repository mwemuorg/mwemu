use crate::emu;
use crate::winapi::helper;

pub fn WSAHtonl(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let hostlong = emu.regs().rdx as u32;
    let result_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAHtonl sock:0x{:x} val:{}", sock, hostlong);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if result_ptr != 0 {
            emu.maps.write_dword(result_ptr, hostlong.to_be());
        }
        emu.regs_mut().rax = 0;
    }
}
