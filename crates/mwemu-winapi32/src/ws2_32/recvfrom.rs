use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn recvfrom(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let buff = arg(emu, 1) as u64;
    let len = arg(emu, 2) as u64;
    log_red!(
        emu,
        "ws2_32!recvfrom sock: {} buff: 0x{:x} len: {}",
        sock,
        buff,
        len
    );
    if !helper::socket_exist(sock as u64) {
        log::trace!("\tinvalid socket.");
        emu.regs_mut().rax = 0xffff_ffff; // SOCKET_ERROR
        return;
    }
    // No real peer: report an orderly connection close (0 bytes).
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 6);
}
