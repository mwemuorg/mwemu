use crate::emu;

pub fn WSAStringToAddressA(emu: &mut emu::Emu) {
    let addr_str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let family = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let sockaddr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let addr_str = if addr_str_ptr != 0 {
        emu.maps.read_string(addr_str_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAStringToAddressA addr:'{}' family:{}",
        addr_str,
        family
    );
    if sockaddr_ptr != 0 && family == 2 {
        emu.maps.write_word(sockaddr_ptr, 2);
        emu.maps.write_word(sockaddr_ptr + 2, 0);
        emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
