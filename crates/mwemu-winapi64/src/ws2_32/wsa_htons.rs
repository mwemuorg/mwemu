use crate::emu;
use crate::winapi::helper;

pub fn WSAHtons(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let hostshort = emu.regs().rdx as u16;
    let result_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSAHtons sock:0x{:x} val:{}", sock, hostshort);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if result_ptr != 0 {
            emu.maps.write_word(result_ptr, hostshort.to_be());
        }
        emu.regs_mut().rax = 0;
    }
}
