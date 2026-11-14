use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn getsockopt(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let level = emu.regs().rdx;
    let optname = emu.regs().r8;
    let optval = emu.regs().r9;
    let optlen = WinApiAbi::arg(emu, 4);

    emu.maps.write_dword(optval, 1);

    log_red!(emu, "ws2_32!getsockopt  lvl: {} opt: {}", level, optname);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
