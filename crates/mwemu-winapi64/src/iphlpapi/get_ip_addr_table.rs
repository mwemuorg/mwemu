use crate::emu;
use super::ERROR_BUFFER_OVERFLOW;

pub(super) fn GetIpAddrTable(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let size = emu.regs().rdx;
    let order = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetIpAddrTable table:0x{:x} order:{}",
        table,
        order
    );

    if size != 0 {
        emu.maps.write_dword(size, 512);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}
