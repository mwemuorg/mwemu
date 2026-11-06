use crate::emu;
use crate::winapi::helper;
pub fn TryAcquireSRWLockExclusive(emu: &mut emu::Emu) {
    let lock_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!TryAcquireSRWLockExclusive lock:0x{:x}",
        lock_ptr
    );
    let handle = helper::handler_create("TryAcquireSrwLockExclusive");
    if lock_ptr != 0 {
        let val = emu.maps.read_dword(lock_ptr).unwrap_or(0);
        if val == 0 {
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
