use crate::emu;
use crate::winapi::helper;

pub(super) fn getsockopt(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let level = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let optname = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let optval = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let optlen = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    emu.maps.write_dword(optval, 1);

    log_red!(emu, "ws2_32!getsockopt  lvl: {} opt: {}", level, optname);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
