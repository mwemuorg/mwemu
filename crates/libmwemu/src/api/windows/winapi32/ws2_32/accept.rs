use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn accept(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let saddr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

    let bytes = emu.maps.read_string_of_bytes(saddr, len as usize);

    log_red!(emu, "ws2_32!accept  connections: {}", bytes);

    stdcall_ret(emu, 3);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
