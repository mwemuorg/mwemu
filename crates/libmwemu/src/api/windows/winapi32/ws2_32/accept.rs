use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn accept(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let saddr = WinApiAbi::arg(emu, 1);
    let len = WinApiAbi::arg(emu, 2);
    let flags = WinApiAbi::arg(emu, 3);

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
