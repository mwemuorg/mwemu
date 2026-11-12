use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

pub fn WSARecvFrom(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buffers_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let buffer_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let bytes_recvd_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
