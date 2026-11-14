use crate::emu;
use crate::winapi::helper;

pub fn getpeername(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let namelen_ptr = emu.regs().r8;
    log_red!(
        emu,
        "ws2_32!getpeername sock: 0x{:x} name: 0x{:x}",
        sock,
        name_ptr
    );
    if !helper::socket_exist(sock) {
        log_red!(emu, "  invalid socket");
        emu.regs_mut().rax = 0xFFFFFFFF; // SOCKET_ERROR
    } else {
        if name_ptr != 0 {
            // Write dummy sockaddr_in: AF_INET, port 80, 127.0.0.1
            emu.maps.write_word(name_ptr, 2); // AF_INET
            emu.maps.write_word(name_ptr + 2, 80u16.to_be()); // port
            emu.maps.write_dword(name_ptr + 4, 0x0100007f); // 127.0.0.1
            for i in 0..8u64 {
                emu.maps.write_byte(name_ptr + 8 + i, 0); // padding
            }
        }
        if namelen_ptr != 0 {
            emu.maps.write_dword(namelen_ptr, 16);
        }
        emu.regs_mut().rax = 0; // success
    }
}
