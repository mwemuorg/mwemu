use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn send(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let buff = WinApiAbi::arg(emu, 1);
    let mut len = WinApiAbi::arg(emu, 2);
    let flags = WinApiAbi::arg(emu, 3);

    let bytes = emu.maps.read_string_of_bytes(buff, len as usize);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    log_red!(emu, "ws2_32!send {{{}}}", bytes);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 0;
        return;
    }

    if emu.cfg.endpoint {
        /*
        let buffer = emu.maps.read_buffer(buff, len as usize);
        let n = endpoint::sock_send(&buffer);
        log::trace!("\tsent {} bytes.", n);
        emu.regs_mut().rax = n as u64;
        */
    } else {
        emu.count_send += 1;
        if emu.count_send > 3 {
            len = 0; // finish the send loop
        }

        emu.regs_mut().rax = len;
    }
}
