use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAJoinLeaf(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let namelen = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(emu, "ws2_32!WSAJoinLeaf sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = sock;
    }
    stdcall_ret(emu, 8);
}
