use crate::emu;
use crate::winapi::helper;

pub fn WSADuplicateSocketA(emu: &mut emu::Emu) {
    wsa_duplicate_socket(emu, "WSADuplicateSocketA");
}

fn wsa_duplicate_socket(emu: &mut emu::Emu, api: &str) {
    let sock = emu.regs().rcx;
    let process_id = emu.regs().rdx;
    let proto_info_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!{api} sock:0x{:x} pid:{}", sock, process_id);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}

pub fn WSADuplicateSocketW(emu: &mut emu::Emu) {
    wsa_duplicate_socket(emu, "WSADuplicateSocketW");
}
