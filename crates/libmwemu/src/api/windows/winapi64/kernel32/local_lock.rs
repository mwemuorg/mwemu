use crate::emu;

pub fn LocalLock(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;

    log_red!(emu, "kernel32!LocalLock hMem:0x{:x}", h_mem);

    // For LMEM_FIXED allocations, the handle is the pointer itself
    emu.set_last_error(0);
    emu.regs_mut().rax = h_mem;
}
