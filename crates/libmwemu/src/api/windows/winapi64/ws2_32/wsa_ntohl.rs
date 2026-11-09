use crate::emu;
use crate::winapi::helper;

pub fn WSANtohl(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let netlong = emu.regs().rdx as u32;
    let result_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSANtohl sock:0x{:x} val:0x{:x}", sock, netlong);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if result_ptr != 0 {
            emu.maps.write_dword(result_ptr, u32::from_be(netlong));
        }
        emu.regs_mut().rax = 0;
    }
}
