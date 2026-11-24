use super::{getrandom, uname};
use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::errors::ENOSYS;
use crate::syscall::linux::constants::syscalls::{
    NR32_CLOCK_GETTIME, NR32_CLOCK_GETTIME64, NR32_GETRANDOM, NR32_GETTIMEOFDAY, NR32_UNAME,
};
use crate::syscall::linux::files;
use crate::syscall::linux::time::{Timespec, clock_gettime, gettimeofday};

mod fs;
mod memory;
mod misc;
mod net;
mod proc;
mod signal;

fn trace_syscall32(emu: &mut emu::Emu, name: &str) {
    log::trace!(
        "{}** {} syscall {} {}",
        emu.colors.light_red,
        emu.pos,
        name,
        emu.colors.nc
    );
}

pub fn gateway(emu: &mut emu::Emu) {
    // INT 0x80 uses the i386 ABI even when the caller executes in long mode.
    let nr = emu.regs().get_eax();
    if emu.resume_due_linux_process() {
        return;
    }
    if !emu.call_syscall_hook(nr) {
        return;
    }
    if files::dispatch(emu, Arch::X86, nr) {
        return;
    }
    if fs::dispatch(emu)
        || proc::dispatch(emu)
        || net::dispatch(emu)
        || memory::dispatch(emu)
        || signal::dispatch(emu)
    {
        return;
    }

    match nr {
        NR32_UNAME => uname(emu, Arch::X86),
        NR32_GETRANDOM => getrandom(emu, Arch::X86),
        NR32_GETTIMEOFDAY => gettimeofday(emu, Arch::X86, Timespec::Time32),
        NR32_CLOCK_GETTIME => clock_gettime(emu, Arch::X86, Timespec::Time32),
        NR32_CLOCK_GETTIME64 => clock_gettime(emu, Arch::X86, Timespec::Time64),
        _ if emu.cfg.is_x64() => {
            // Legacy i386 operations have no checked long-mode frame or pointer-width contract.
            log::trace!("unsupported Linux i386 syscall {nr} in long mode");
            LinuxSyscall.set_return(emu, ENOSYS);
        }
        _ => misc::dispatch(emu),
    }
}
