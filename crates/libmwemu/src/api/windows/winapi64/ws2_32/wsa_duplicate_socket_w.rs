use crate::emu;
use crate::winapi::helper;

pub fn WSADuplicateSocketW(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let process_id = emu.regs().rdx;
    let proto_info_ptr = emu.regs().r8;
    log_red!(
        emu,
        "ws2_32!WSADuplicateSocketW sock:0x{:x} pid:{}",
        sock,
        process_id
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}
