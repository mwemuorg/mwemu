use crate::emu;
use crate::winapi::helper::{self, WinApiAbi};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

pub fn WSAConnect(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let namelen = emu.regs().r8 as u32;
    let caller_data_ptr = emu.regs().r9;
    let callee_data_ptr = WinApiAbi::arg(emu, 4);
    let sqos_ptr = WinApiAbi::arg(emu, 5);
    let gqos_ptr = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "ws2_32!WSAConnect sock:0x{:x} name:0x{:x} namelen:{} callerdata:0x{:x} calleedata:0x{:x} sqos:0x{:x} gqos:0x{:x}",
        sock,
        name_ptr,
        namelen,
        caller_data_ptr,
        callee_data_ptr,
        sqos_ptr,
        gqos_ptr
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(WSAENOTSOCK);
        emu.regs_mut().rax = SOCKET_ERROR;
    } else {
        if callee_data_ptr != 0 {
            emu.maps.write_qword(callee_data_ptr, 0);
            emu.maps.write_qword(callee_data_ptr + 8, 0);
        }
        emu.regs_mut().rax = 0;
    }
}
