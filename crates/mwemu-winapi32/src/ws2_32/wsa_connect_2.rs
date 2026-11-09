use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WsaConnect(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let name_ptr = arg(emu, 1) as u64;
    log_red!(emu, "ws2_32!WSAConnect sock:{} name:0x{:x}", sock, name_ptr);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xffff_ffff_u64;
    } else {
        if name_ptr != 0 {
            let port = emu.maps.read_word(name_ptr + 2).unwrap_or(0).to_be();
            let ip0 = emu.maps.read_byte(name_ptr + 4).unwrap_or(0);
            let ip1 = emu.maps.read_byte(name_ptr + 5).unwrap_or(0);
            let ip2 = emu.maps.read_byte(name_ptr + 6).unwrap_or(0);
            let ip3 = emu.maps.read_byte(name_ptr + 7).unwrap_or(0);
            log_red!(
                emu,
                "\tconnecting to {}.{}.{}.{}:{}",
                ip0,
                ip1,
                ip2,
                ip3,
                port
            );
        }
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 7);
}
