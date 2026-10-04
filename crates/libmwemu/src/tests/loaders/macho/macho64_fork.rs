//! End-to-end fork()/pthread programs (sources next to the fixtures). Each
//! fixture checks itself and exits 0 on success. They need the real dylibs
//! extracted by `make dyld`, and skip when those are absent.
use crate::tests::helpers;
use crate::*;

const FORK: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork.bin");
const PTHREAD: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_pthread.bin");
const FORK_EDGE: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_edge.bin");
const FORK_HEAP: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_heap.bin");

fn real_dylibs_present() -> bool {
    ["maps/macos/aarch64", "../../maps/macos/aarch64"]
        .iter()
        .any(|d| std::path::Path::new(d).join("libsystem_c.dylib").exists())
}

/// Run a fixture to completion and return the finished emulator, or `None`
/// when the real dylibs are not available.
fn run_fixture(name: &str, bytes: &[u8], memory_guard: bool) -> Option<Emu> {
    if !real_dylibs_present() {
        eprintln!("skipping {}: run `make dyld` to extract macOS dylibs", name);
        return None;
    }
    helpers::setup();
    let tmp = std::env::temp_dir().join(format!("mwemu_test_{}.bin", name));
    std::fs::write(&tmp, bytes).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());
    emu.set_memory_guard(memory_guard);
    let _ = emu.run(None);
    Some(emu)
}

/// Wait status of a fixture run without the memory guard.
fn exit_status(name: &str, bytes: &[u8]) -> Option<u64> {
    let emu = run_fixture(name, bytes, false)?;
    Some(emu.processes.exit_status.expect("program did not exit"))
}

#[test]
fn macho64_fork_children_compute_and_are_reaped() {
    if let Some(status) = exit_status("macho64_aarch64_fork", FORK) {
        assert_eq!(status, 0);
    }
}

#[test]
fn macho64_pthread_create_join() {
    if let Some(status) = exit_status("macho64_aarch64_pthread", PTHREAD) {
        assert_eq!(status, 0);
    }
}

#[test]
fn macho64_fork_edge_cases() {
    if let Some(status) = exit_status("macho64_aarch64_fork_edge", FORK_EDGE) {
        assert_eq!(status, 0);
    }
}

#[test]
fn macho64_fork_child_inherits_heap() {
    if let Some(status) = exit_status("macho64_aarch64_fork_heap", FORK_HEAP) {
        assert_eq!(status, 0);
    }
}

#[test]
fn macho64_fork_heap_has_no_memory_guard_findings() {
    if let Some(emu) = run_fixture("macho64_aarch64_fork_heap_guard", FORK_HEAP, true) {
        assert_eq!(emu.processes.exit_status, Some(0));
        let findings = &emu.kernel.as_ref().expect("guard ledger").findings;
        assert!(findings.is_empty(), "false findings: {}", findings.len());
    }
}
