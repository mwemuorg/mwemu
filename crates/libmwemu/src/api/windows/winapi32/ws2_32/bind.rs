use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn bind(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let saddr = WinApiAbi::arg(emu, 1);
    let len = WinApiAbi::arg(emu, 2);

    let family: u16 = emu.maps.read_word(saddr).unwrap_or(0);
    let port: u16 = emu.maps.read_word(saddr + 2).unwrap_or(0);
    let ip: u32 = emu.maps.read_dword(saddr + 4).unwrap_or(0);

    let sip = format!(
        "{}.{}.{}.{}",
        ip & 0xff,
        (ip & 0xff00) >> 8,
        (ip & 0xff0000) >> 16,
        (ip & 0xff000000) >> 24
    );

    log_red!(
        emu,
        "ws2_32!bind  family: {} {}:{}",
        family,
        sip,
        port.to_be()
    );

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if !helper::socket_exist(sock) {
        log::trace!("\tbad socket.");
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
