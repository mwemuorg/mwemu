//! End-to-end fork()/pthread programs (sources next to the fixtures). Each
//! fixture checks itself and exits 0 on success. They need the real dylibs
//! extracted by `make dyld`.
use crate::tests::helpers;
use crate::*;

const FORK: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork.bin");
const PTHREAD: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_pthread.bin");
const FORK_EDGE: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_edge.bin");
const FORK_HEAP: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_heap.bin");
const MUTEX: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_mutex.bin");
const CONDVAR: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_condvar.bin");
const FORK_ATFORK: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_atfork.bin");
const FORK_HELD_LOCK: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_held_lock.bin");

fn real_dylibs_present() -> bool {
    ["maps/macos/aarch64", "../../maps/macos/aarch64"]
        .iter()
        .any(|d| std::path::Path::new(d).join("libsystem_c.dylib").exists())
}

fn run_fixture(
    name: &str,
    bytes: &[u8],
    memory_guard: bool,
) -> (Emu, Result<u64, crate::err::MwemuError>) {
    assert!(real_dylibs_present(), "missing macOS dylibs; run make dyld");
    helpers::setup();
    let tmp = std::env::temp_dir().join(format!("mwemu_test_{}.bin", name));
    std::fs::write(&tmp, bytes).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());
    emu.set_memory_guard(memory_guard);
    let result = emu.run(None);
    (emu, result)
}

/// Wait status of a fixture run without the memory guard.
fn exit_status(name: &str, bytes: &[u8]) -> u64 {
    let (emu, _) = run_fixture(name, bytes, false);
    emu.processes.exit_status.expect("program did not exit")
}

#[test]
fn macho64_fork_children_compute_and_are_reaped() {
    assert_eq!(exit_status("macho64_aarch64_fork", FORK), 0);
}

#[test]
fn macho64_pthread_create_join() {
    assert_eq!(exit_status("macho64_aarch64_pthread", PTHREAD), 0);
}

#[test]
fn macho64_fork_edge_cases() {
    assert_eq!(exit_status("macho64_aarch64_fork_edge", FORK_EDGE), 0);
}

#[test]
fn macho64_fork_child_inherits_heap() {
    assert_eq!(exit_status("macho64_aarch64_fork_heap", FORK_HEAP), 0);
}

#[test]
fn macho64_fork_heap_has_no_memory_guard_findings() {
    let (emu, _) = run_fixture("macho64_aarch64_fork_heap_guard", FORK_HEAP, true);
    assert_eq!(emu.processes.exit_status, Some(0));
    let findings = &emu.kernel.as_ref().expect("guard ledger").findings;
    assert!(findings.is_empty(), "false findings: {}", findings.len());
}

#[test]
fn macho64_mutex_types_unfair_lock_once_and_llsc() {
    assert_eq!(exit_status("macho64_aarch64_mutex", MUTEX), 0);
}

#[test]
fn macho64_condvar_producer_consumer() {
    assert_eq!(exit_status("macho64_aarch64_condvar", CONDVAR), 0);
}

#[test]
fn macho64_fork_runs_atfork_handlers_in_posix_order() {
    assert_eq!(exit_status("macho64_aarch64_fork_atfork", FORK_ATFORK), 0);
}

#[test]
fn macho64_fork_with_lock_held_by_other_thread_deadlocks_child() {
    let (emu, result) = run_fixture("macho64_aarch64_fork_held_lock", FORK_HELD_LOCK, false);
    let err = result.expect_err("the child's lock() must not return");
    assert!(err.to_string().contains("blocked"), "{}", err);
    assert!(
        emu.processes.exit_status.is_none(),
        "program must not finish"
    );
    assert!(
        emu.processes.in_child(),
        "deadlock must happen in the child"
    );
    assert!(emu.threads[0].blocked_on_lock.is_some());
}
