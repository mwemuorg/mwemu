use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::process;
use crate::syscall::linux::tls32;
use crate::syscall::linux::{self, unsupported};

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() {
        NR32_GETUID | NR32_GETGID | NR32_GETEUID | NR32_GETEGID | NR32_GETUID32 | NR32_GETGID32
        | NR32_GETEUID32 | NR32_GETEGID32 => linux::get_identity(emu),
        NR32_GETGROUPS => linux::get_groups(emu, Arch::X86, true),
        NR32_GETGROUPS32 => linux::get_groups(emu, Arch::X86, false),
        NR32_GETRESUID | NR32_GETRESGID => linux::get_res_ids(emu, Arch::X86, true),
        NR32_GETRESUID32 | NR32_GETRESGID32 => linux::get_res_ids(emu, Arch::X86, false),
        NR32_GETPID => linux::getpid(emu),
        NR32_GETPPID => linux::getppid(emu),
        NR32_GETTID => linux::gettid(emu),
        NR32_SET_THREAD_AREA => tls32::thread_area(emu, false),
        NR32_GET_THREAD_AREA => tls32::thread_area(emu, true),
        NR32_SET_TID_ADDRESS => linux::set_tid_address(emu, Arch::X86),
        NR32_RESTART_SYSCALL => unsupported(emu, "restart_syscall"),
        NR32_EXIT | NR32_EXIT_GROUP => process::exit(emu),
        NR32_FORK => process::fork(emu, Arch::X86),
        NR32_NANOSLEEP => process::nanosleep(emu),
        NR32_WAITPID => process::wait_child(emu, Arch::X86),
        NR32_EXECVE => unsupported(emu, "execve"),
        NR32_TIME => unsupported(emu, "time"),
        _ => return false,
    }

    true
}
