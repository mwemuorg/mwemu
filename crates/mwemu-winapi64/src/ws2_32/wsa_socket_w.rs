use crate::emu;
use crate::winapi::helper;

pub fn WSASocketW(emu: &mut emu::Emu) {
    let af = emu.regs().rcx as u32;
    let sock_type = emu.regs().rdx as u32;
    let proto = emu.regs().r8 as u32;
    log_red!(
        emu,
        "ws2_32!WSASocketW af:{} type:{} proto:{}",
        af,
        sock_type,
        proto
    );
    let sock = helper::socket_create();
    emu.set_last_error(0);
    emu.regs_mut().rax = sock;
}
