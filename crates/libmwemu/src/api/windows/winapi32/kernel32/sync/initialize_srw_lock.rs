use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InitializeSRWLock(emu: &mut emu::Emu) {
    let lock_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!InitializeSRWLock ptr: 0x{:x}", lock_ptr);
    emu.maps.write_dword(lock_ptr, 0);
    stdcall_ret(emu, 1);
}
