use crate::emu;

pub fn ReleaseSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!ReleaseSRWLockShared ptr: 0x{:x}", lock_ptr);
}
