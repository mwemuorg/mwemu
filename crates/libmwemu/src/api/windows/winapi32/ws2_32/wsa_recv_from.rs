use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

pub fn WSARecvFrom(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let buffers_ptr = WinApiAbi::arg(emu, 1);
    let buffer_count = WinApiAbi::arg(emu, 2) as u32;
    let bytes_recvd_ptr = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "ws2_32!WSARecvFrom sock:0x{:x} buffers:{}",
        sock,
        buffer_count
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(WSAENOTSOCK);
        emu.regs_mut().rax = SOCKET_ERROR;
    } else {
        if bytes_recvd_ptr != 0 {
            emu.maps.write_dword(bytes_recvd_ptr, 0);
        }
        emu.regs_mut().rax = 0;
    }
    for _ in 0..9u32 {
        emu.stack_pop32(false);
    }
}
