use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSAEnumProtocolsA(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsA");
}

fn wsa_enum_protocols(emu: &mut emu::Emu, api: &str) {
    let protocols_ptr = WinApiAbi::arg(emu, 0);
    let buf_ptr = WinApiAbi::arg(emu, 1);
    let buf_len_ptr = WinApiAbi::arg(emu, 2);
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
