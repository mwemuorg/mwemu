use crate::emu;

pub fn AcquireSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!AcquireSRWLockShared ptr: 0x{:x}", lock_ptr);
}
