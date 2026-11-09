use crate::emu;
use crate::winapi::helper;

pub fn listen(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let connections = emu.regs().rdx;

    log_red!(emu, "ws2_32!listen  connections: {}", connections);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
