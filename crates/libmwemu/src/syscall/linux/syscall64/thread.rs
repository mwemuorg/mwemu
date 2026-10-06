//! Linux thread and process-control syscalls on the emulator's thread
//! scheduler: `clone` creates a `ThreadContext`, `futex` blocks and wakes
//! threads by address, `exit` ends only the calling thread, and `gettid` /
//! `set_tid_address` / `sched_yield` report or yield the current one.
//!
//! A vfork-style `clone` (CLONE_VFORK | CLONE_VM, what `os/exec` and glibc's
//! `posix_spawn` use) runs the child as a thread on the parent's stack while
//! the parent sleeps; the child's `execve` is logged with its argv and ends
//! that thread, so the program sees a spawned process that finished, and
//! `wait4` reports it exited 0. Go's runtime (M threads, notes and
//! semaphores on futexes) is the reference workload.

use crate::emu;
use crate::windows::constants;

const CLONE_VM: u64 = 0x100;
const CLONE_VFORK: u64 = 0x4000;
const CLONE_SETTLS: u64 = 0x80000;
const CLONE_PARENT_SETTID: u64 = 0x100000;
const CLONE_CHILD_CLEARTID: u64 = 0x200000;
const CLONE_CHILD_SETTID: u64 = 0x1000000;

const FUTEX_WAIT: u64 = 0;
const FUTEX_WAKE: u64 = 1;
const FUTEX_WAIT_BITSET: u64 = 9;
const FUTEX_WAKE_BITSET: u64 = 10;
const EAGAIN: u64 = -11i64 as u64;
const ENOSYS: u64 = -38i64 as u64;

/// `syscall` is always `0f 05`: the child resumes right after it.
const SYSCALL_INSN_LEN: u64 = 2;

/// A vfork parent blocks on this synthetic "address" (no user mapping lives
/// there) until the child tid it encodes calls `execve` or `exit`.
const VFORK_WAIT_KEY: u64 = 0xffff_ffff_0000_0000;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_CLONE => clone(emu),
        constants::NR64_FUTEX => futex(emu),
        constants::NR64_EXIT => exit_thread(emu),
        constants::NR64_EXECVE => execve(emu),
        constants::NR64_WAIT4 => wait4(emu),
        constants::NR64_WAITID => waitid(emu),
        // No pidfds: Go (1.23+) and glibc fall back to wait4/waitid on ENOSYS.
        constants::NR64_PIDFD_OPEN => ret0_named(emu, ENOSYS, "pidfd_open"),
        constants::NR64_GETTID => gettid(emu),
        constants::NR64_GETPPID => ret0_named(emu, 1, "getppid"),
        constants::NR64_PRCTL => ret0_named(emu, 0, "prctl"),
        constants::NR64_DUP3 => dup3(emu),
        constants::NR64_SET_TID_ADDRESS => set_tid_address(emu),
        constants::NR64_SCHED_YIELD => sched_yield(emu),
        _ => return false,
    }
    true
}

fn current_tid(emu: &emu::Emu) -> u64 {
    emu.threads[emu.current_thread_id].id
}

/// Threads still able to run or be woken (an exited thread stays in the
/// vector, suspended, because the scheduler indexes by position).
fn live_threads(emu: &emu::Emu) -> usize {
    emu.threads.iter().filter(|t| !t.suspended).count()
}

/// Park the current thread until all others are blocked or sleeping, then
/// let it run again: the emulator's tick only advances when nothing else is
/// runnable, so this is "lowest priority", which is what a yield, a sleep or
/// a timed wait want from a scheduler without real time.
pub(super) fn yield_current(emu: &mut emu::Emu) {
    if live_threads(emu) > 1 {
        let tick = emu.tick;
        emu.threads[emu.current_thread_id].wake_tick = tick + 1;
    }
}

/// Wake up to `max` threads blocked on `uaddr`; returns how many woke.
fn wake_waiters(emu: &mut emu::Emu, uaddr: u64, max: u64) -> u64 {
    let mut woken = 0;
    for thread in emu.threads.iter_mut() {
        if woken == max {
            break;
        }
        if thread.blocked_on_cs == Some(uaddr) {
            thread.blocked_on_cs = None;
            thread.wake_tick = 0;
            woken += 1;
        }
    }
    woken
}

/// End the current thread. A CLONE_CHILD_CLEARTID / set_tid_address word is
/// zeroed and its futex waiters woken (how pthread_join and Go's thread
/// teardown learn the thread is gone), and a vfork parent is released.
fn retire_current_thread(emu: &mut emu::Emu) {
    let tid = current_tid(emu);
    let clear = emu.threads[emu.current_thread_id].clear_child_tid;
    if clear != 0 {
        emu.maps.write_dword(clear, 0);
        wake_waiters(emu, clear, u64::MAX);
    }
    wake_waiters(emu, VFORK_WAIT_KEY | tid, u64::MAX);
    emu.threads[emu.current_thread_id].suspended = true;
}

fn ret0_named(emu: &mut emu::Emu, rax: u64, name: &str) {
    emu.regs_mut().rax = rax;
    super::trace_syscall64(emu, name);
}

/// clone(flags, child_stack, parent_tid, child_tid, tls): with CLONE_VM the
/// child is a new emulated thread sharing memory; it resumes after the
/// `syscall` with rax = 0 on its own stack (or the parent's, for vfork, in
/// which case the parent sleeps until the child execs or exits). Without
/// CLONE_VM (fork-style) there is no address space to copy, so the parent
/// just gets a child pid.
fn clone(emu: &mut emu::Emu) {
    let flags = emu.regs().rdi;
    let stack = emu.regs().rsi;
    let parent_tid_ptr = emu.regs().rdx;
    let child_tid_ptr = emu.regs().r10;
    let tls = emu.regs().r8;

    if flags & CLONE_VM == 0 {
        emu.regs_mut().rax = 1001;
        super::misc::trace_syscall64_args(
            emu,
            "clone",
            &[(
                "flags",
                format!("0x{flags:x} (fork-style, no child emulated)"),
            )],
        );
        return;
    }

    let tid = 0x1000 + emu.threads.len() as u64;
    let mut child = emu.threads[emu.current_thread_id].clone();
    child.id = tid;
    child.suspended = false;
    child.wake_tick = 0;
    child.blocked_on_cs = None;
    child.handle = 0;
    child.clear_child_tid = if flags & CLONE_CHILD_CLEARTID != 0 {
        child_tid_ptr
    } else {
        0
    };
    {
        let regs = child.regs_x86_mut();
        regs.rax = 0;
        regs.rip += SYSCALL_INSN_LEN;
        if stack != 0 {
            regs.rsp = stack;
        }
        if flags & CLONE_SETTLS != 0 {
            regs.fs = tls;
        }
    }
    if flags & CLONE_PARENT_SETTID != 0 {
        emu.maps.write_dword(parent_tid_ptr, tid as u32);
    }
    if flags & CLONE_CHILD_SETTID != 0 {
        emu.maps.write_dword(child_tid_ptr, tid as u32);
    }
    emu.threads.push(child);
    if flags & CLONE_VFORK != 0 {
        emu.threads[emu.current_thread_id].blocked_on_cs = Some(VFORK_WAIT_KEY | tid);
    }
    // A second thread exists now: the run loop hands over to the scheduler.
    emu.cfg.enable_threading = true;
    emu.regs_mut().rax = tid;

    super::misc::trace_syscall64_args(
        emu,
        "clone",
        &[
            ("flags", format!("0x{flags:x}")),
            ("stack", format!("0x{stack:x}")),
            ("tid", tid.to_string()),
        ],
    );
}

/// futex(uaddr, op, val, timeout, ...): WAIT blocks the current thread on
/// `uaddr` when the word still holds `val` (EAGAIN otherwise); WAKE releases
/// up to `val` threads blocked there. Other ops report "nobody woken".
fn futex(emu: &mut emu::Emu) {
    let uaddr = emu.regs().rdi;
    let op = emu.regs().rsi & 0x7f; // strip PRIVATE / CLOCK_REALTIME flags
    let val = emu.regs().rdx as u32;
    let timeout = emu.regs().r10;

    let rax = match op {
        FUTEX_WAIT | FUTEX_WAIT_BITSET => {
            let cur = emu.maps.read_dword(uaddr).unwrap_or(0);
            if cur != val {
                EAGAIN
            } else if live_threads(emu) <= 1 {
                // Single-threaded: nobody can ever wake us, so this is a lock
                // its own owner — us — never released after an emulation
                // hiccup. Drop it to the free state and report a wake so
                // glibc's `while(xchg(lock,2)) futex_wait` loop re-acquires.
                let _ = emu.maps.write_dword(uaddr, 0);
                0
            } else if timeout != 0 {
                // Timed wait: a spurious wake-up is legal, so just yield and
                // let the caller re-check its deadline.
                yield_current(emu);
                0
            } else {
                emu.threads[emu.current_thread_id].blocked_on_cs = Some(uaddr);
                0
            }
        }
        FUTEX_WAKE | FUTEX_WAKE_BITSET => wake_waiters(emu, uaddr, val as u64),
        _ => 0,
    };
    emu.regs_mut().rax = rax;

    super::misc::trace_syscall64_args(
        emu,
        "futex",
        &[
            ("uaddr", format!("0x{uaddr:x}")),
            ("op", op.to_string()),
            ("val", val.to_string()),
            ("result", format!("{}", rax as i64)),
        ],
    );
}

/// exit(status) ends the calling thread only; the last live thread ends the
/// process like exit_group.
fn exit_thread(emu: &mut emu::Emu) {
    if live_threads(emu) <= 1 {
        super::proc::handle_syscall64_exit(emu);
        return;
    }
    let status = emu.regs().rdi;
    let tid = current_tid(emu);
    retire_current_thread(emu);
    super::misc::trace_syscall64_args(
        emu,
        "exit",
        &[("tid", tid.to_string()), ("status", status.to_string())],
    );
}

/// Read a NULL-terminated `char *argv[]` from guest memory.
fn read_string_array(emu: &emu::Emu, mut ptr: u64) -> Vec<String> {
    let mut out = Vec::new();
    while ptr != 0 {
        let Some(item) = emu.maps.read_qword(ptr) else {
            break;
        };
        if item == 0 || out.len() >= 64 {
            break;
        }
        out.push(emu.maps.read_string(item));
        ptr += 8;
    }
    out
}

/// execve(path, argv, envp): the command line is what an analyst wants, so
/// it is logged whole. In a vfork child the thread is retired (its image
/// would be replaced) and the parent resumes; elsewhere the call "succeeds"
/// and execution continues, as before.
fn execve(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rdi);
    let argv = read_string_array(emu, emu.regs().rsi);
    log::info!("execve: {} {:?}", path, argv);
    super::misc::trace_syscall64_args(
        emu,
        "execve",
        &[("path", path), ("argv", format!("{argv:?}"))],
    );

    let key = VFORK_WAIT_KEY | current_tid(emu);
    let is_vfork_child = emu.threads.iter().any(|t| t.blocked_on_cs == Some(key));
    if is_vfork_child {
        retire_current_thread(emu);
    }
    emu.regs_mut().rax = 0;
}

/// wait4(pid, *status, options, *rusage): every child we ever report has
/// already finished (exec'd children are retired on the spot), so answer
/// "exited with 0" right away.
fn wait4(emu: &mut emu::Emu) {
    let pid = emu.regs().rdi as i64;
    let status_ptr = emu.regs().rsi;
    let tid = if pid > 0 {
        pid as u64
    } else {
        emu.threads
            .iter()
            .filter(|t| t.suspended)
            .map(|t| t.id)
            .next_back()
            .unwrap_or(1001)
    };
    if status_ptr != 0 {
        emu.maps.write_dword(status_ptr, 0);
    }
    emu.regs_mut().rax = tid;
    super::misc::trace_syscall64_args(emu, "wait4", &[("pid", tid.to_string())]);
}

/// waitid(idtype, id, *siginfo, options, *rusage): like `wait4`, the child
/// is already gone, so fill a CLD_EXITED siginfo with status 0.
fn waitid(emu: &mut emu::Emu) {
    const SIGCHLD: u32 = 17;
    const CLD_EXITED: u32 = 1;
    let id = emu.regs().rsi;
    let info = emu.regs().rdx;
    if info != 0 {
        emu.maps.write_dword(info, SIGCHLD); // si_signo
        emu.maps.write_dword(info + 4, 0); // si_errno
        emu.maps.write_dword(info + 8, CLD_EXITED); // si_code
        emu.maps.write_dword(info + 16, id as u32); // si_pid
        emu.maps.write_dword(info + 20, 1000); // si_uid
        emu.maps.write_dword(info + 24, 0); // si_status
    }
    emu.regs_mut().rax = 0;
    super::misc::trace_syscall64_args(emu, "waitid", &[("id", id.to_string())]);
}

/// dup3(oldfd, newfd, flags): descriptors are not rewired (stdout stays the
/// host's stdout), the call just succeeds with `newfd`.
fn dup3(emu: &mut emu::Emu) {
    let old = emu.regs().rdi;
    let new = emu.regs().rsi;
    emu.regs_mut().rax = new;
    super::misc::trace_syscall64_args(
        emu,
        "dup3",
        &[("oldfd", old.to_string()), ("newfd", new.to_string())],
    );
}

fn gettid(emu: &mut emu::Emu) {
    emu.regs_mut().rax = current_tid(emu);
    super::trace_syscall64(emu, "gettid");
}

fn set_tid_address(emu: &mut emu::Emu) {
    let ptr = emu.regs().rdi;
    emu.threads[emu.current_thread_id].clear_child_tid = ptr;
    emu.regs_mut().rax = current_tid(emu);
    super::misc::trace_syscall64_args(emu, "set_tid_address", &[("tidptr", format!("0x{ptr:x}"))]);
}

fn sched_yield(emu: &mut emu::Emu) {
    yield_current(emu);
    emu.regs_mut().rax = 0;
    super::trace_syscall64(emu, "sched_yield");
}
