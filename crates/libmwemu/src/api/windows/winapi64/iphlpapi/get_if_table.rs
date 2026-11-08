use crate::emu;
use super::ERROR_BUFFER_OVERFLOW;

pub(super) fn GetIfTable(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let size = emu.regs().rdx;
    let order = emu.regs().r8;

    log_red!(
        emu,
        "iphlpapi!GetIfTable table:0x{:x} order:{}",
        table,
        order
    );

    if size != 0 {
        emu.maps.write_dword(size, 1024);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}
