use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn TryAcquireSRWLockExclusive(emu: &mut emu::Emu) {
    let lock_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!TryAcquireSRWLockExclusive lock:0x{:x}",
        lock_ptr
    );
    if lock_ptr != 0 {
        let val = emu.maps.read_dword(lock_ptr).unwrap_or(0);
        if val == 0 {
            emu.maps.write_dword(lock_ptr, 1);
            emu.regs_mut().rax = 1;
        } else {
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 1);
}
