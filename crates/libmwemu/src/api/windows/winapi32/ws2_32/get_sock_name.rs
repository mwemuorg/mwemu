use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetSockName(emu: &mut emu::Emu) {
    let sock = arg(emu, 0);
    let sockaddr_ptr = arg(emu, 1) as u64;
    let namelen_ptr = arg(emu, 2) as u64;
    log_red!(emu, "ws2_32!GetSockName sock: {}", sock);
    if sockaddr_ptr != 0 {
        emu.maps.write_dword(sockaddr_ptr, 0);
    }
    if namelen_ptr != 0 {
        emu.maps.write_dword(namelen_ptr, 16); // sizeof(sockaddr_in)
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
