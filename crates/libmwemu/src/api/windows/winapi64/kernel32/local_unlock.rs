use crate::emu;

pub fn LocalUnlock(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;

    log_red!(emu, "kernel32!LocalUnlock hMem:0x{:x}", h_mem);

    // For LMEM_FIXED allocations, unlock returns FALSE and sets ERROR_NOT_LOCKED
    emu.set_last_error(158); // ERROR_NOT_LOCKED
    emu.regs_mut().rax = 0; // FALSE
}
