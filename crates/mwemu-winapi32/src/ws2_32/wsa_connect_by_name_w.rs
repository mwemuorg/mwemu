use crate::emu;
use crate::winapi::helper;

pub fn WSAConnectByNameW(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let node_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let node = if node_ptr != 0 {
        emu.maps.read_wide_string(node_ptr)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        emu.maps.read_wide_string(service_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAConnectByNameW sock:0x{:x} node:'{}' service:'{}'",
        sock,
        node,
        service
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = 1;
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
