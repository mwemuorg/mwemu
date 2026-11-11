use crate::emu;

pub fn WSAStringToAddressA(emu: &mut emu::Emu) {
    let addr_str_ptr = emu.regs().rcx;
    let family = emu.regs().rdx as u32;
    let proto_info_ptr = emu.regs().r8;
    let sockaddr_ptr = emu.regs().r9;
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
}

pub fn WSAStringToAddressW(emu: &mut emu::Emu) {
    let addr_str_ptr = emu.regs().rcx;
    let family = emu.regs().rdx as u32;
    let proto_info_ptr = emu.regs().r8;
    let sockaddr_ptr = emu.regs().r9;
    let addr_str = if addr_str_ptr != 0 {
        emu.maps.read_wide_string(addr_str_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAStringToAddressW addr:'{}' family:{}",
        addr_str,
        family
    );
    if sockaddr_ptr != 0 && family == 2 {
        emu.maps.write_word(sockaddr_ptr, 2);
        emu.maps.write_word(sockaddr_ptr + 2, 0);
        emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f);
    }
    emu.regs_mut().rax = 0;
}
