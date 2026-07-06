use crate::debug::console::Console;
use crate::emu;
use crate::windows::constants;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_RESTART_SYSCALL => handle_syscall64_restart(emu),
        constants::NR64_EXIT => handle_syscall64_exit(emu),
        constants::NR64_FORK => handle_syscall64_fork(emu),
        constants::NR64_KILL => handle_syscall64_kill(emu),
        constants::NR64_DUP => handle_syscall64_dup(emu),
        constants::NR64_DUP2 => handle_syscall64_dup2(emu),
        _ => return false,
    }

    true
}

pub(super) fn handle_syscall64_restart(emu: &mut emu::Emu) {
    super::trace_syscall64(emu, "restart_syscall");
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

pub(super) fn handle_syscall64_fork(emu: &mut emu::Emu) {
    super::trace_syscall64(emu, "fork");
    Console::spawn_console(emu);
}

pub(super) fn handle_syscall64_kill(emu: &mut emu::Emu) {
    let pid = emu.regs().rdi;
    let sig = emu.regs().rsi;
    log::trace!(
        "{}** {} syscall kill() pid: {} sig: {} {}",
        emu.colors.light_red,
        emu.pos,
        pid,
        sig,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_dup(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    log::trace!(
        "{}** {} syscall dup() fd: {} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_dup2(emu: &mut emu::Emu) {
    let old_fd = emu.regs().rdi;
    let new_fd = emu.regs().rsi;
    log::trace!(
        "{}** {} syscall dup2() oldfd: {} newfd: {} {}",
        emu.colors.light_red,
        emu.pos,
        old_fd,
        new_fd,
        emu.colors.nc
    );
}
