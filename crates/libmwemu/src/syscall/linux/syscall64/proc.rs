use super::thread::{retire_current_thread, vfork_parent};
use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::{process, unsupported};
use crate::threading::process::exit_status;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        NR64_RESTART_SYSCALL => unsupported(emu, "restart_syscall"),
        NR64_FORK => process::fork(emu, Arch::X86_64),
        NR64_KILL => unsupported(emu, "kill"),
        NR64_DUP => unsupported(emu, "dup"),
        NR64_DUP2 => unsupported(emu, "dup2"),
        _ => return false,
    }

    true
}

pub(super) fn handle_syscall64_exit(emu: &mut emu::Emu) {
    log::trace!(
        "{}** {} syscall exit()  {} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rdi,
        emu.colors.nc
    );
    let status = emu.regs().rdi;
    if vfork_parent(emu).is_some() {
        retire_current_thread(emu, status);
    } else if emu.process_exit(exit_status(status)).is_some() {
        emu.force_reload = true;
    }
}
