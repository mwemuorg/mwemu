use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn listen(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let connections = WinApiAbi::arg(emu, 1);

    log_red!(emu, "ws2_32!listen  connections: {}", connections);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
