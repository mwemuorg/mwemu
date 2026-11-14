use crate::emu;

pub fn ReleaseSRWLockExclusive(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!ReleaseSRWLockExclusive lock:0x{:x}",
        lock_ptr
    );
    if lock_ptr != 0 {
        emu.maps.write_dword(lock_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
