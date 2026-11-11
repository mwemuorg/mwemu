use crate::emu;
use crate::winapi::helper;

pub(super) fn WSASocketA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WsaSocketA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );

    emu.regs_mut().rax = helper::socket_create();
}

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

pub fn WsaSocketA(emu: &mut emu::Emu) {
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
