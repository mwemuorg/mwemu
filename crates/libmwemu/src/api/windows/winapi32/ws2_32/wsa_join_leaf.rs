use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAJoinLeaf(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let namelen = WinApiAbi::arg(emu, 2) as u32;
    log_red!(emu, "ws2_32!WSAJoinLeaf sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = sock;
    }
    stdcall_ret(emu, 8);
}
