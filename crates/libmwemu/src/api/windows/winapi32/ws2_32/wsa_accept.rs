use crate::emu;
use crate::winapi::helper;

pub fn WSAAccept(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let addrlen_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let condition_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAAccept sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        let new_sock = helper::socket_create();
        if addr_ptr != 0 {
            emu.maps.write_word(addr_ptr, 2);
            emu.maps.write_word(addr_ptr + 2, 0x5000);
            emu.maps.write_dword(addr_ptr + 4, 0x0100007f);
        }
        if addrlen_ptr != 0 {
            emu.maps.write_dword(addrlen_ptr, 16);
        }
        emu.regs_mut().rax = new_sock;
    }
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
}
