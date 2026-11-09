use crate::emu;
use crate::winapi::helper;

pub(super) fn WsaAccept(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let saddr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cond = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let callback = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(
        emu,
        "ws2_32!WsaAccept  connections: {} callback: {}",
        bytes,
        callback
    );

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
