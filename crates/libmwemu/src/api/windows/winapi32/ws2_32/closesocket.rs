use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn closesocket(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);

    log_red!(emu, "ws2_32!closesocket");

    helper::socket_close(sock);

    /*
    if emu.cfg.endpoint {
        endpoint::sock_close();
    }*/

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
