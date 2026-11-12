use crate::emu;

pub fn AcquireSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!AcquireSRWLockShared lock:0x{:x}", lock_ptr);
    if lock_ptr != 0 {
        let val = emu.maps.read_dword(lock_ptr).unwrap_or(0);
        emu.maps.write_dword(lock_ptr, val.wrapping_add(1));
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
