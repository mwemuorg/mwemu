//! Clock syscalls backed by the host clock: programs that sleep, time out or
//! schedule timers (Go's runtime does all three) need `nanotime()` to move.

use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;

use crate::syscall::linux::time::{Timespec, clock_gettime, gettimeofday};

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        NR64_CLOCK_GETTIME => clock_gettime(emu, Arch::X86_64, Timespec::Time64),
        NR64_GETTIMEOFDAY => gettimeofday(emu, Arch::X86_64, Timespec::Time64),
        _ => return false,
    }
    true
}
