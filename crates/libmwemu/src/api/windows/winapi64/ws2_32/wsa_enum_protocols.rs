use crate::emu;

pub fn WSAEnumProtocolsA(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsA");
}

fn wsa_enum_protocols(emu: &mut emu::Emu, api: &str) {
    let protocols_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!{api}");
    if buf_len_ptr != 0 {
        emu.maps.write_dword(buf_len_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}

pub fn WSAEnumProtocolsW(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsW");
}
