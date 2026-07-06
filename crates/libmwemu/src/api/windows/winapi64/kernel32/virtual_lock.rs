use crate::emu;

pub fn VirtualLock(emu: &mut emu::Emu) {
    let lp_address = emu.regs().rcx;
    let dw_size = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!VirtualLock addr: 0x{:x} size: 0x{:x}",
        lp_address,
        dw_size
    );

    // TODO: Implement actual memory locking functionality
    // VirtualLock locks pages in physical memory to prevent paging to disk
    // For emulation purposes, this is typically not critical

    emu.regs_mut().rax = 1; // Return TRUE (non-zero)
}
