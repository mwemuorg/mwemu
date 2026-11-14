use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn shutdown(emu: &mut emu::Emu) {
    let sock = arg(emu, 0) as u64;
    let how = arg(emu, 1);
    log_red!(emu, "ws2_32!shutdown sock:{} how:{}", sock, how);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        emu.regs_mut().rax = 0xffff_ffff_u64;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 2);
}
