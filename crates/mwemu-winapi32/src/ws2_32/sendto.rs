use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn sendto(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let len = arg(emu, 2);
    log_red!(emu, "ws2_32!sendto sock: {} len: {}", sock, len);
    if !helper::socket_exist(sock as u64) {
        log::trace!("\tinvalid socket.");
    }
    emu.regs_mut().rax = len as u64; // bytes sent
    stdcall_ret(emu, 6);
}
