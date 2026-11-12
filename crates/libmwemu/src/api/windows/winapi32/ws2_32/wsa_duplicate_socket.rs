use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSADuplicateSocketA(emu: &mut emu::Emu) {
    wsa_duplicate_socket(emu, "WSADuplicateSocketA");
}

fn wsa_duplicate_socket(emu: &mut emu::Emu, api: &str) {
    let sock = WinApiAbi::arg(emu, 0);
    let process_id = WinApiAbi::arg(emu, 1) as u32;
    let proto_info_ptr = WinApiAbi::arg(emu, 2);
    log_red!(emu, "ws2_32!{api} sock:0x{:x} pid:{}", sock, process_id);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSADuplicateSocketW(emu: &mut emu::Emu) {
    wsa_duplicate_socket(emu, "WSADuplicateSocketW");
}
