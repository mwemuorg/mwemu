use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSAHtonl(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let hostlong = WinApiAbi::arg(emu, 1) as u32;
    let result_ptr = WinApiAbi::arg(emu, 2);
    log_red!(emu, "ws2_32!WSAHtonl sock:0x{:x} val:{}", sock, hostlong);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if result_ptr != 0 {
            emu.maps.write_dword(result_ptr, hostlong.to_be());
        }
        emu.regs_mut().rax = 0;
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
