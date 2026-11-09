use crate::emu;
use crate::winapi::helper;

pub fn WSAAddressToStringW(emu: &mut emu::Emu) {
    let addr_ptr = emu.regs().rcx;
    let addr_len = emu.regs().rdx as u32;
    let proto_info_ptr = emu.regs().r8;
    let str_ptr = emu.regs().r9;
    log_red!(emu, "ws2_32!WSAAddressToStringW addr:0x{:x}", addr_ptr);
    let handle = helper::handler_create("WSAAddressToStringW");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    if addr_ptr != 0 && str_ptr != 0 {
        let ip = emu.maps.read_dword(addr_ptr + 4).unwrap_or(0x0100007f);
        let port = emu.maps.read_word(addr_ptr + 2).unwrap_or(0).to_be();
        let s = format!(
            "{}.{}.{}.{}:{}",
            ip & 0xFF,
            (ip >> 8) & 0xFF,
            (ip >> 16) & 0xFF,
            (ip >> 24) & 0xFF,
            port
        );
        emu.maps.write_wide_string(str_ptr, &s);
    }
    emu.regs_mut().rax = 0;
}
