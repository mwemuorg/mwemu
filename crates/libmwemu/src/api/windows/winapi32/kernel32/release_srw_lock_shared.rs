use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ReleaseSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!ReleaseSRWLockShared ptr: 0x{:x}", lock_ptr);
    stdcall_ret(emu, 1);
}
