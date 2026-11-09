use crate::emu;
use crate::winapi::helper;

pub fn WSAConnectByList(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let addr_list_ptr = emu.regs().rdx;
    let local_addr_len_ptr = emu.regs().r8;
    let local_addr_ptr = emu.regs().r9;
    let remote_addr_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let remote_addr_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let timeout_ptr = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _reserved = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WSAConnectByList sock:0x{:x} addrlist:0x{:x} localaddr:0x{:x} remoteaddr:0x{:x}",
        sock,
        addr_list_ptr,
        local_addr_ptr,
        remote_addr_ptr
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0;
    } else {
        if local_addr_len_ptr != 0 {
            emu.maps.write_dword(local_addr_len_ptr, 16);
        }
        if local_addr_ptr != 0 {
            emu.maps.write_dword(local_addr_ptr, 2);
        }
        if remote_addr_len_ptr != 0 {
            emu.maps.write_dword(remote_addr_len_ptr, 16);
        }
        if remote_addr_ptr != 0 {
            emu.maps.write_dword(remote_addr_ptr, 2);
        }
        emu.regs_mut().rax = 1;
    }
}
