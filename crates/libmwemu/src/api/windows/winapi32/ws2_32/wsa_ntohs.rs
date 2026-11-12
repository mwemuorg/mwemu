use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSANtohs(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let netshort = WinApiAbi::arg(emu, 1) as u32 as u16;
    let result_ptr = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "ws2_32!WSANtohs sock:0x{:x} val:0x{:x}",
        sock,
        netshort
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if result_ptr != 0 {
            emu.maps.write_word(result_ptr, u16::from_be(netshort));
        }
        emu.regs_mut().rax = 0;
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
