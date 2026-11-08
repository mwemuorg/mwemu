use crate::emu;
use super::NO_ERROR;

pub(super) fn GetBestRoute(emu: &mut emu::Emu) {
    let dest_addr = emu.regs().rcx;
    let source_addr = emu.regs().rdx;
    let best_route = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetBestRoute dest:0x{:x} src:0x{:x} pBestRoute:0x{:x}",
        dest_addr,
        source_addr,
        best_route
    );

    // Fill minimal MIB_IPFORWARDROW (76 bytes) with zeroes if pointer is valid
    if best_route != 0 {
        for i in 0..19 {
            emu.maps.write_dword(best_route + i * 4, 0);
        }
    }

    emu.regs_mut().rax = NO_ERROR;
}
