use crate::emu;

pub fn getsockname(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let namelen_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!getsockname sock:0x{:x}", sock);
    if name_ptr != 0 {
        emu.maps.write_word(name_ptr, 2); // AF_INET
        emu.maps.write_word(name_ptr + 2, 0); // port 0
        emu.maps.write_dword(name_ptr + 4, 0); // INADDR_ANY
        for i in 0..8u64 {
            emu.maps.write_byte(name_ptr + 8 + i, 0);
        }
    }
    if namelen_ptr != 0 {
        emu.maps.write_dword(namelen_ptr, 16);
    }
    emu.regs_mut().rax = 0;
}
