use crate::emu;
use crate::winapi::helper;

pub fn WSASendDisconnect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let data_ptr = emu.regs().rdx;
    log_red!(emu, "ws2_32!WSASendDisconnect sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
}
