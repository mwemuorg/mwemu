use super::trace_syscall32;
use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
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
        NR32_SET_TID_ADDRESS => linux::set_tid_address(emu, Arch::X86),
        0 => unsupported(emu, "restart_syscall"),
        1 => handle_syscall32_exit(emu),
        2 => unsupported(emu, "fork"),
        7 => unsupported(emu, "waitpid"),
        11 => unsupported(emu, "execve"),
        13 => unsupported(emu, "time"),
        _ => return false,
    }

    true
}

fn handle_syscall32_exit(emu: &mut emu::Emu) {
    trace_syscall32(emu, "exit");
    emu.stop();
}
