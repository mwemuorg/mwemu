use crate::emu;

pub(super) fn WsaHtons(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let host_port = emu.regs().rdx as u16;
    let out_port = emu.regs().r8;

    let net_port = host_port.to_be();
    log_red!(emu, "ws2_32!WsaHtons {} -> 0x{:x}", host_port, net_port);

    if out_port != 0 {
        emu.maps.write_word(out_port, net_port);
    }
    emu.regs_mut().rax = 0;
}
