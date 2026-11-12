use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSASendMsg(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let msg_ptr = WinApiAbi::arg(emu, 1);
    let flags = WinApiAbi::arg(emu, 2) as u32;
    let bytes_sent_ptr = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "ws2_32!WSASendMsg sock:0x{:x} flags:0x{:x}",
        sock,
        flags
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        if bytes_sent_ptr != 0 {
            emu.maps.write_dword(bytes_sent_ptr, 0);
        }
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 6);
}
