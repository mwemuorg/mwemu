use crate::emu;
use crate::winapi::helper;

pub fn ioctlsocket(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let cmd = emu.regs().rdx as u32;
    let argp_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!ioctlsocket sock:0x{:x} cmd:0x{:x}", sock, cmd);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if argp_ptr != 0 && cmd == 0x4004667f {
            emu.maps.write_dword(argp_ptr, 0);
        }
        emu.regs_mut().rax = 0;
    }
}
