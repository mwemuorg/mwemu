//! Bounded epoll registration model. Pipe queues and eventfd counters use shared I/O handlers.

use crate::arch::Arch;

use super::misc::trace_syscall64_args;
use super::thread::yield_current;
use crate::api::windows::helper;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::io;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        NR64_EPOLL_CREATE | NR64_EPOLL_CREATE1 => new_fd(emu, "epoll:", "epoll_create"),
        NR64_EVENTFD => io::eventfd(emu, Arch::X86_64, false),
        NR64_EVENTFD2 => io::eventfd(emu, Arch::X86_64, true),
        NR64_EPOLL_CTL => epoll_ctl(emu),
        NR64_EPOLL_WAIT | NR64_EPOLL_PWAIT | NR64_EPOLL_PWAIT2 => epoll_wait(emu),
        NR64_PIPE => io::pipe(emu, Arch::X86_64, false),
        NR64_PIPE2 => io::pipe(emu, Arch::X86_64, true),
        _ => return false,
    }
    true
}

fn new_fd(emu: &mut emu::Emu, uri: &str, name: &str) {
    let fd = helper::handler_create(uri);
    emu.regs_mut().rax = fd;
    trace_syscall64_args(emu, name, &[("fd", fd.to_string())]);
}

/// epoll_ctl(epfd, op, fd, *event): registrations are accepted and forgotten.
fn epoll_ctl(emu: &mut emu::Emu) {
    let epfd = emu.regs().rdi;
    let op = emu.regs().rsi;
    let fd = emu.regs().rdx;
    emu.regs_mut().rax = 0;
    trace_syscall64_args(
        emu,
        "epoll_ctl",
        &[
            ("epfd", epfd.to_string()),
            ("op", op.to_string()),
            ("fd", fd.to_string()),
        ],
    );
}

/// epoll_wait / epoll_pwait / epoll_pwait2: no source is ever ready. Yield
/// first so the threads this one is implicitly waiting for get to run.
fn epoll_wait(emu: &mut emu::Emu) {
    let epfd = emu.regs().rdi;
    yield_current(emu);
    emu.regs_mut().rax = 0;
    trace_syscall64_args(
        emu,
        "epoll_wait",
        &[("epfd", epfd.to_string()), ("events", "0".to_string())],
    );
}
