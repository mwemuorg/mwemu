use crate::emu;
use crate::winapi::helper;

pub fn WSAConnectByNameA(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let node_ptr = emu.regs().rdx;
    let service_ptr = emu.regs().r8;
    let node = if node_ptr != 0 {
        emu.maps.read_string(node_ptr)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        emu.maps.read_string(service_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!WSAConnectByNameA sock:0x{:x} node:'{}' service:'{}'",
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
