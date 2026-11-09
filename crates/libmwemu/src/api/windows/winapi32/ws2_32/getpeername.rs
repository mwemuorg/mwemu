use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn getpeername(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    log_red!(emu, "ws2_32!getpeername sock: {}", sock);
    emu.regs_mut().rax = 0xffff_ffff_u64; // SOCKET_ERROR
    stdcall_ret(emu, 3);
}
