use crate::emu;
use crate::winapi::helper;

pub fn WSAAddressToStringA(emu: &mut emu::Emu) {
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let addr_len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!WSAAddressToStringA addr:0x{:x}", addr_ptr);
    let handle = helper::handler_create("WSAAddressToStringA");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, handle as u32);
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
        emu.maps.write_string(str_ptr, &s);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
