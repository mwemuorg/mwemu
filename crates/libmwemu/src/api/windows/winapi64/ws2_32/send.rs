use crate::emu;
use crate::winapi::helper;

pub fn send(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buff = emu.regs().rdx;
    let mut len = emu.regs().r8;
    let flags = emu.regs().r9;

    let bytes = emu.maps.read_string_of_bytes(buff, len as usize);

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
