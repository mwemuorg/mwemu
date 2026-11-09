use crate::emu;
use crate::winapi::helper;

pub fn closesocket(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;

    log_red!(emu, "ws2_32!closesocket");

    helper::socket_close(sock);

    /*
    if emu.cfg.endpoint {
        endpoint::sock_close();
    }*/

    emu.regs_mut().rax = 0;
}
