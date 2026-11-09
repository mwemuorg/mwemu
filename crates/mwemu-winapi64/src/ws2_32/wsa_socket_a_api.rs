use crate::emu;
use crate::winapi::helper;

pub fn WsaSocketA_api(emu: &mut emu::Emu) {
    let af = emu.regs().rcx as u32;
    let sock_type = emu.regs().rdx as u32;
    let protocol = emu.regs().r8 as u32;
    let sock = helper::socket_create();
    log_red!(
        emu,
        "ws2_32!WsaSocketA af:{} type:{} proto:{} => 0x{:x}",
        af,
        sock_type,
        protocol,
        sock
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = sock;
}
