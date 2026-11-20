use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::unsupported;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        NR64_RESTART_SYSCALL => unsupported(emu, "restart_syscall"),
        NR64_FORK => unsupported(emu, "fork"),
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
    emu.stop();
}
