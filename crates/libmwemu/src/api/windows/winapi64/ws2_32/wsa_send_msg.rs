use crate::emu;
use crate::winapi::helper;

pub fn WSASendMsg(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let msg_ptr = emu.regs().rdx;
    let flags = emu.regs().r8 as u32;
    let bytes_sent_ptr = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSASendMsg sock:0x{:x} flags:0x{:x}",
        sock,
        flags
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if bytes_sent_ptr != 0 {
            emu.maps.write_dword(bytes_sent_ptr, 0);
        }
        emu.regs_mut().rax = 0;
    }
}
