//! Event-loop plumbing as pseudo file descriptors: epoll instances, eventfds
//! and pipes live in the handle table with a `scheme:` URI, so `read` /
//! `write` / `close` already accept them (a read returns EOF, a write is
//! swallowed). Nothing ever becomes ready: `epoll_wait` reports no events
//! after yielding, which is what an emulated program with no I/O sources
//! should see, and timers keep working because the clock still advances.

use crate::api::windows::helper;
use crate::emu;
use crate::windows::constants;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_EPOLL_CREATE | constants::NR64_EPOLL_CREATE1 => {
            new_fd(emu, "epoll:", "epoll_create")
        }
        constants::NR64_EVENTFD | constants::NR64_EVENTFD2 => new_fd(emu, "eventfd:", "eventfd"),
        constants::NR64_EPOLL_CTL => epoll_ctl(emu),
        constants::NR64_EPOLL_WAIT | constants::NR64_EPOLL_PWAIT | constants::NR64_EPOLL_PWAIT2 => {
            epoll_wait(emu)
        }
        constants::NR64_PIPE | constants::NR64_PIPE2 => pipe(emu),
        _ => return false,
    }
    true
}

fn new_fd(emu: &mut emu::Emu, uri: &str, name: &str) {
    let fd = helper::handler_create(uri);
    emu.regs_mut().rax = fd;
    super::misc::trace_syscall64_args(emu, name, &[("fd", fd.to_string())]);
}

/// epoll_ctl(epfd, op, fd, *event): registrations are accepted and forgotten.
fn epoll_ctl(emu: &mut emu::Emu) {
    let epfd = emu.regs().rdi;
    let op = emu.regs().rsi;
    let fd = emu.regs().rdx;
    emu.regs_mut().rax = 0;
    super::misc::trace_syscall64_args(
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
    super::thread::yield_current(emu);
    emu.regs_mut().rax = 0;
    super::misc::trace_syscall64_args(
        emu,
        "epoll_wait",
        &[("epfd", epfd.to_string()), ("events", "0".to_string())],
    );
}

/// pipe / pipe2(*fds, flags): a read end that reports EOF and a write end
/// that swallows data.
fn pipe(emu: &mut emu::Emu) {
    let fds = emu.regs().rdi;
    let read_end = helper::handler_create("pipe:r");
    let write_end = helper::handler_create("pipe:w");
    emu.maps.write_dword(fds, read_end as u32);
    emu.maps.write_dword(fds + 4, write_end as u32);
    emu.regs_mut().rax = 0;
    super::misc::trace_syscall64_args(
        emu,
        "pipe",
        &[("r", read_end.to_string()), ("w", write_end.to_string())],
    );
}
