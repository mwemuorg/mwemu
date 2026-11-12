use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSAEnumNetworkEvents(emu: &mut emu::Emu) {
    let sock = WinApiAbi::arg(emu, 0);
    let event = WinApiAbi::arg(emu, 1);
    let net_events_ptr = WinApiAbi::arg(emu, 2);
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
