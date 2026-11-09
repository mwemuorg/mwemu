use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn WsaRecvFrom(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let bytes_recv_ptr = arg(emu, 3) as u64;
    log_red!(emu, "ws2_32!WSARecvFrom sock: {}", sock);
    if bytes_recv_ptr != 0 {
        emu.maps.write_dword(bytes_recv_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 9);
}
