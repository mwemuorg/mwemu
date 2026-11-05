use crate::emu;

pub fn TryAcquireSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!TryAcquireSRWLockShared lock:0x{:x}",
        lock_ptr
    );
    if lock_ptr != 0 {
        let val = emu.maps.read_dword(lock_ptr).unwrap_or(0);
        emu.maps.write_dword(lock_ptr, val.wrapping_add(0x10));
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
