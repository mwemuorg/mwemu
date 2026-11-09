use crate::emu;
use crate::winapi::helper;

pub fn closesocket(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "ws2_32!closesocket");

    helper::socket_close(sock);

    /*
    if emu.cfg.endpoint {
        endpoint::sock_close();
    }*/

    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
