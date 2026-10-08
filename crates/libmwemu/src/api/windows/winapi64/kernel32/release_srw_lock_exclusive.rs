use crate::emu;

pub fn ReleaseSRWLockExclusive(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!ReleaseSRWLockExclusive ptr: 0x{:x}",
        lock_ptr
    );
}
