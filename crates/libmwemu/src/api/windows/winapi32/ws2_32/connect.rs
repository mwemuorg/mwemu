use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn connect(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let sockaddr_ptr = WinApiAbi::arg(emu, 1);
    //let sockaddr = emu.maps.read_bytes(sockaddr_ptr, 8);
    let family: u16 = emu.maps.read_word(sockaddr_ptr).unwrap_or(0);
    let port: u16 = emu.maps.read_word(sockaddr_ptr + 2).unwrap_or(0).to_be();
    let ip: u32 = emu.maps.read_dword(sockaddr_ptr + 4).unwrap_or(0);

    let sip = format!(
        "{}.{}.{}.{}",
        ip & 0xff,
        (ip & 0xff00) >> 8,
        (ip & 0xff0000) >> 16,
        (ip & 0xff000000) >> 24
    );
    log_red!(emu, "ws2_32!connect  family: {} {}:{}", family, sip, port);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if emu.cfg.endpoint {
        /*
        if endpoint::sock_connect(sip.as_str(), port) {
            log::trace!("\tconnected to the endpoint.");
        } else {
            log::trace!("\tcannot connect. dont use -e");
        }*/
        emu.regs_mut().rax = 0;
    } else {
        // offline mode

        if !helper::socket_exist(sock) {
            log::trace!("\tinvalid socket.");
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    }
}
