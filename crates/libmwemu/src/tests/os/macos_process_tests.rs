//! macOS fork/wait/pthread APIs, called directly through the libsystem gateway.
use super::unix_api_helpers::{aarch64_env, ret, set_args};
use crate::api::macos::libsystem::gateway;
use crate::emu::Emu;
use crate::threading::process::INITIAL_PID;

#[test]
fn fork_runs_child_then_resumes_parent_with_child_pid() {
    let (mut emu, _) = aarch64_env();
    emu.regs_aarch64_mut().pc = 0x1000_0000;

    gateway("_fork", &mut emu);
    assert_eq!(ret(&emu), 0, "child sees fork() == 0");
    gateway("_getpid", &mut emu);
    let child = ret(&emu);
    assert_ne!(child, INITIAL_PID);
    gateway("_getppid", &mut emu);
    assert_eq!(ret(&emu), INITIAL_PID);

    emu.regs_aarch64_mut().pc = 0x2000_0000; // child wanders off
    set_args(&mut emu, &[5]);
    gateway("__exit", &mut emu);

    assert_eq!(ret(&emu), child, "parent sees fork() == child pid");
    assert_eq!(emu.regs_aarch64().pc, 0x1000_0000, "parent pc restored");
    assert!(
        emu.processes.exit_status.is_none(),
        "root process still alive"
    );
    gateway("_getpid", &mut emu);
    assert_eq!(ret(&emu), INITIAL_PID);
}

#[test]
fn child_memory_writes_do_not_leak_into_parent() {
    let (mut emu, d) = aarch64_env();
    emu.maps.write_qword(d, 0x1111);

    gateway("_fork", &mut emu);
    emu.maps.write_qword(d, 0x2222);
    set_args(&mut emu, &[0]);
    gateway("__exit", &mut emu);

    assert_eq!(emu.maps.read_qword(d), Some(0x1111));
}

#[test]
fn waitpid_reaps_exit_status_once() {
    let (mut emu, d) = aarch64_env();
    gateway("_fork", &mut emu);
    set_args(&mut emu, &[0x1ff]);
    gateway("__exit", &mut emu);
    let child = ret(&emu);

    set_args(&mut emu, &[child, d, 0]);
    gateway("_waitpid", &mut emu);
    assert_eq!(ret(&emu), child);
    assert_eq!(emu.maps.read_dword(d), Some(0xff00), "WEXITSTATUS == 0xff");

    set_args(&mut emu, &[-1i64 as u64, d, 0]);
    gateway("_waitpid", &mut emu);
    assert_eq!(ret(&emu), -1i64 as u64, "no children left");
}

#[test]
fn abort_in_child_reports_sigabrt() {
    let (mut emu, d) = aarch64_env();
    gateway("_fork", &mut emu);
    gateway("_abort", &mut emu);
    set_args(&mut emu, &[d]);
    gateway("_wait", &mut emu);
    assert_eq!(emu.maps.read_dword(d), Some(6), "WTERMSIG == SIGABRT");
}

#[test]
fn root_exit_records_status_and_stops() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[3]);
    gateway("_exit", &mut emu);
    assert_eq!(emu.processes.exit_status, Some(0x300));
    assert!(emu.process_terminated);
}

#[test]
fn pthread_join_unknown_thread_is_esrch() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[0xdead, 0]);
    gateway("_pthread_join", &mut emu);
    assert_eq!(ret(&emu), 3);
}

#[test]
fn pthread_self_is_main_thread_id() {
    let (mut emu, _) = aarch64_env();
    gateway("_pthread_self", &mut emu);
    assert_eq!(ret(&emu), emu.threads[0].id);
}

// --- thread synchronisation ------------------------------------------------

use crate::threading::context::ThreadContext;

/// Add a second thread and return its index.
fn second_thread(emu: &mut Emu) -> usize {
    let t = ThreadContext::new(0x1001, emu.cfg.arch);
    emu.threads.push(t);
    emu.threads.len() - 1
}

#[test]
fn mutex_lock_blocks_then_unlock_hands_over_ownership() {
    let (mut emu, m) = aarch64_env();
    let other = second_thread(&mut emu);
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_lock", &mut emu); // main owns it

    emu.current_thread_id = other;
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_lock", &mut emu);
    assert_eq!(emu.threads[other].blocked_on_lock, Some(m));

    emu.current_thread_id = 0;
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_unlock", &mut emu);
    assert_eq!(emu.threads[other].blocked_on_lock, None);
    assert_eq!(emu.threads[other].regs_aarch64().x[0], 0);
    assert_eq!(
        emu.maps.read_qword(m + 8),
        Some(0x1001),
        "owner handed over"
    );
}

#[test]
fn mutex_trylock_is_ebusy_when_held_by_another_thread() {
    let (mut emu, m) = aarch64_env();
    let other = second_thread(&mut emu);
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_lock", &mut emu);
    emu.current_thread_id = other;
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_trylock", &mut emu);
    assert_eq!(ret(&emu), 16);
}

#[test]
fn cond_signal_moves_waiter_to_the_mutex() {
    let (mut emu, d) = aarch64_env();
    let (c, m) = (d, d + 0x100);
    let other = second_thread(&mut emu);

    emu.current_thread_id = other;
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_lock", &mut emu);
    set_args(&mut emu, &[c, m]);
    gateway("_pthread_cond_wait", &mut emu);
    assert_eq!(emu.threads[other].cond_wait, Some((c, m)));
    assert_eq!(
        emu.maps.read_qword(m + 8),
        Some(0),
        "wait released the mutex"
    );

    emu.current_thread_id = 0;
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_lock", &mut emu);
    set_args(&mut emu, &[c]);
    gateway("_pthread_cond_signal", &mut emu);
    assert_eq!(emu.threads[other].cond_wait, None);
    assert_eq!(
        emu.threads[other].blocked_on_lock,
        Some(m),
        "waits for the mutex"
    );
    set_args(&mut emu, &[m]);
    gateway("_pthread_mutex_unlock", &mut emu);
    assert!(emu.threads[other].is_runnable(emu.tick));
}

#[test]
fn atfork_child_handler_list_survives_fork() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[0, 0, 0]);
    gateway("_pthread_atfork", &mut emu);
    gateway("_fork", &mut emu);
    assert_eq!(emu.processes.atfork.len(), 1);
}

#[test]
fn raw_fork_exit_restores_the_parent_and_reports_the_child_status() {
    use crate::syscall::macos::{syscall_aarch64, syscall_x86_64};
    use crate::{emu_aarch64, emu64};
    for mut emu in [emu64(), emu_aarch64()] {
        let syscall = |emu: &mut Emu, nr, arg| {
            if emu.cfg.arch.is_aarch64() {
                let r = emu.regs_aarch64_mut();
                r.x[16] = nr;
                r.x[0] = arg;
                r.nzcv.c = true;
                syscall_aarch64::gateway(emu);
                let r = emu.regs_aarch64();
                (r.x[0], r.x[1], r.nzcv.c)
            } else {
                emu.regs_mut().rax = 0x2000000 | nr;
                emu.regs_mut().rdi = arg;
                emu.flags_mut().f_cf = true;
                syscall_x86_64::gateway(emu);
                (emu.regs().rax, emu.regs().rdx, emu.flags().f_cf)
            }
        };
        let parent = emu.processes.pid;
        let result = syscall(&mut emu, 2, 0);
        let child = emu.processes.pid;
        assert_ne!(child, parent);
        assert_eq!(result, (child, 1, false));
        assert_eq!(syscall(&mut emu, 1, 7), (child, 0, false));
        assert_eq!(emu.processes.pid, parent);
        assert_eq!(emu.reap_child(child as i64), Some((child, 7 << 8)));
        syscall(&mut emu, 1, 3);
        assert_eq!(emu.processes.exit_status, Some(3 << 8));
    }
}

#[test]
fn raw_bsd_calls_return_errno_in_carry_and_write_counted_bytes() {
    use crate::maps::mem64::Permission;
    use crate::syscall::macos::{syscall_aarch64, syscall_x86_64};
    use crate::{emu_aarch64, emu64};
    const DATA: u64 = 0x10000;
    for mut emu in [emu64(), emu_aarch64()] {
        emu.maps
            .create_map("data", DATA, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_bytes(DATA, b"a\0b");
        let call = |emu: &mut Emu, nr, args: [u64; 6]| {
            if emu.cfg.arch.is_aarch64() {
                let r = emu.regs_aarch64_mut();
                r.x[16] = nr;
                r.x[..6].copy_from_slice(&args);
                syscall_aarch64::gateway(emu);
                let r = emu.regs_aarch64();
                (r.x[0], r.nzcv.c)
            } else {
                let r = emu.regs_mut();
                r.rax = nr;
                [r.rdi, r.rsi, r.rdx, r.r10, r.r8, r.r9] = args;
                syscall_x86_64::gateway(emu);
                (emu.regs().rax, emu.flags().f_cf)
            }
        };
        assert_eq!(call(&mut emu, 4, [13, DATA, 3, 0, 0, 0]), (9, true));
        assert_eq!(call(&mut emu, 4, [1, DATA, 3, 0, 0, 0]), (3, false));
        assert_eq!(emu.emulated_stdout, b"a\0b");
        assert_eq!(call(&mut emu, 4, [1, 0, 1, 0, 0, 0]), (14, true));
        assert_eq!(call(&mut emu, 0xffff, [0; 6]), (78, true));
        let (map, failed) = call(&mut emu, 197, [0, 4096, 3, 0x1002, u64::MAX, 0]);
        assert!(!failed);
        assert!(emu.maps.write_byte(map, 1));
        assert_eq!(call(&mut emu, 74, [map, 4096, 1, 0, 0, 0]), (0, false));
        assert!(!emu.maps.can_access_range(map, 1, Permission::WRITE));
        assert_eq!(call(&mut emu, 73, [map, 4096, 0, 0, 0, 0]), (0, false));
        assert!(emu.maps.get_mem_by_addr(map).is_none());
    }
}
