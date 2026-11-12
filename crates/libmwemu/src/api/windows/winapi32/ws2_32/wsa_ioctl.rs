use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAIoctl(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let ioctl_code = WinApiAbi::arg(emu, 1) as u32;
    let in_buf_ptr = WinApiAbi::arg(emu, 2);
    let in_buf_len = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!WSAIoctl sock:0x{:x} code:0x{:x}",
        sock,
        ioctl_code
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 9);
}
