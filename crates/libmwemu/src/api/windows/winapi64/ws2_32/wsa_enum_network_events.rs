use crate::emu;

pub fn WSAEnumNetworkEvents(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let event = emu.regs().rdx;
    let net_events_ptr = emu.regs().r8;
    log_red!(
        emu,
        "ws2_32!WSAEnumNetworkEvents sock: 0x{:x} event: 0x{:x}",
        sock,
        event
    );
    if net_events_ptr != 0 {
        // Zero out WSANETWORKEVENTS structure (4 + 10*4 = 44 bytes)
        for i in 0..44u64 {
            emu.maps.write_byte(net_events_ptr + i, 0);
        }
        // Set FD_READ | FD_WRITE | FD_CONNECT in lNetworkEvents
        emu.maps.write_dword(net_events_ptr, 0x01 | 0x02 | 0x10);
    }
    emu.regs_mut().rax = 0;
}
