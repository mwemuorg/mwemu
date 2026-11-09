use crate::emu;

pub fn WSAEnumNetworkEvents(emu: &mut emu::Emu) {
    let sock = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let event = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let net_events_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "ws2_32!WSAEnumNetworkEvents sock:0x{:x} event:0x{:x}",
        sock,
        event
    );
    if net_events_ptr != 0 {
        for i in 0..44u64 {
            emu.maps.write_byte(net_events_ptr + i, 0);
        }
        emu.maps.write_dword(net_events_ptr, 0x01 | 0x02 | 0x10);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
