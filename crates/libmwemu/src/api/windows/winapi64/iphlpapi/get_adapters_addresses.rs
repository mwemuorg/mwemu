use crate::emu;
use super::ERROR_BUFFER_OVERFLOW;

pub(super) fn GetAdaptersAddresses(emu: &mut emu::Emu) {
    let family = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let adapter_addresses = emu.regs().r9;
    let size_pointer = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .unwrap_or(0);

    log_red!(
        emu,
        "iphlpapi!GetAdaptersAddresses family:{} flags:0x{:x} pAddresses:0x{:x}",
        family,
        flags,
        adapter_addresses
    );

    if size_pointer != 0 {
        emu.maps.write_dword(size_pointer, 640);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}
