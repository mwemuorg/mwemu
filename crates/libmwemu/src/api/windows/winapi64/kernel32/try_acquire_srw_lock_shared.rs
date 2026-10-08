use crate::emu;

pub fn TryAcquireSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!TryAcquireSRWLockShared ptr: 0x{:x}",
        lock_ptr
    );

    emu.regs_mut().rax = 1;
}
