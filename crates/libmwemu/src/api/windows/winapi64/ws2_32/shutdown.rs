use crate::emu;
use crate::winapi::helper;

pub fn shutdown(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let how = emu.regs().rdx as u32;
    log_red!(emu, "ws2_32!shutdown sock:0x{:x} how:{}", sock, how);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}
