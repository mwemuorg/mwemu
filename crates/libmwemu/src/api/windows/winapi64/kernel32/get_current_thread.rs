use crate::emu;

pub fn GetCurrentThread(emu: &mut emu::Emu) {
    log_red!(emu, "** {} kernel32!GetCurrentThread", emu.pos);

    // GetCurrentThread returns a pseudo-handle (-2 or 0xFFFFFFFFFFFFFFFE in 64-bit)
    // This is a special constant that always refers to the current thread
    const CURRENT_THREAD_PSEUDO_HANDLE: u64 = 0xFFFFFFFFFFFFFFFE; // -2 as u64

    emu.regs_mut().rax = CURRENT_THREAD_PSEUDO_HANDLE;
}
