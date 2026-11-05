use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn AcquireSRWLockExclusive(emu: &mut emu::Emu) {
    let lock_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!AcquireSRWLockExclusive lock:0x{:x}",
        lock_ptr
    );
    if lock_ptr != 0 {
        emu.maps.write_dword(lock_ptr, 1);
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 1);
}
