//! macOS fork/wait/pthread APIs, called directly through the libsystem gateway.
use super::unix_api_helpers::{aarch64_env, ret, set_args};
use crate::api::macos::libsystem::gateway;
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
