use crate::emu;

pub fn WsaStartup(emu: &mut emu::Emu) {
    let version = emu.regs().rcx as u16;
    let wsadata_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSAStartup version:0x{:x} data:0x{:x}",
        version,
        wsadata_ptr
    );
    if wsadata_ptr != 0 {
        emu.maps.write_word(wsadata_ptr, version);
        emu.maps.write_word(wsadata_ptr + 2, version);
        emu.maps.write_string(wsadata_ptr + 4, "WinSock 2.0");
        emu.maps.write_string(wsadata_ptr + 4 + 257, "Running");
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129, 0x0200);
        emu.maps.write_word(wsadata_ptr + 4 + 257 + 129 + 2, 0x0101);
    }
    emu.regs_mut().rax = 0;
}
