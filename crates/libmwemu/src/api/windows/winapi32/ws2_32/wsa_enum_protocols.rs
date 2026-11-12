use crate::emu;

pub fn WSAEnumProtocolsA(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsA");
}

fn wsa_enum_protocols(emu: &mut emu::Emu, api: &str) {
    let protocols_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!{api}");
    if buf_len_ptr != 0 {
        emu.maps.write_dword(buf_len_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSAEnumProtocolsW(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsW");
}
