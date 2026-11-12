use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn TryAcquireSRWLockShared(emu: &mut emu::Emu) {
    let lock_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!TryAcquireSRWLockShared ptr: 0x{:x}",
        lock_ptr
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
