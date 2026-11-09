use crate::emu;

pub(super) fn WsaHtons(emu: &mut emu::Emu) {
    let _sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let host_port = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u16;
    let out_port_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    log_red!(emu, "ws2_32!WsaHtons port:{}", host_port);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if out_port_ptr != 0 {
        emu.maps.write_word(out_port_ptr, host_port.to_be());
    }
    emu.regs_mut().rax = 0;
}
