use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn WSAConnectByNameA(emu: &mut emu::Emu) {
    wsa_connect_by_name(emu, "WSAConnectByNameA", false);
}

fn wsa_connect_by_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let sock = emu.regs().rcx;
    let node_ptr = emu.regs().rdx;
    let service_ptr = emu.regs().r8;
    let node = if node_ptr != 0 {
        strings::read_text(emu, node_ptr, wide)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        strings::read_text(emu, service_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!{api} sock:0x{:x} node:'{}' service:'{}'",
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
}

pub fn WSAConnectByNameW(emu: &mut emu::Emu) {
    wsa_connect_by_name(emu, "WSAConnectByNameW", true);
}
