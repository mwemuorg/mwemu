use crate::emu;

pub fn AcquireSRWLockExclusive(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!AcquireSRWLockExclusive ptr: 0x{:x}",
        lock_ptr
    );
}
