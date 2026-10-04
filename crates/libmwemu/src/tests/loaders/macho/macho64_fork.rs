//! End-to-end fork()/pthread programs (sources next to the fixtures). Each
//! fixture checks itself and exits 0 on success. They need the real dylibs
//! extracted by `make dyld`, and skip when those are absent.
use crate::tests::helpers;
use crate::*;

const FORK: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork.bin");
const PTHREAD: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_pthread.bin");
const FORK_EDGE: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_fork_edge.bin");

fn real_dylibs_present() -> bool {
    ["maps/macos/aarch64", "../../maps/macos/aarch64"]
        .iter()
        .any(|d| std::path::Path::new(d).join("libsystem_c.dylib").exists())
}

/// Run a fixture to completion and return its wait status, or `None` when
/// the real dylibs are not available.
fn run_fixture(name: &str, bytes: &[u8]) -> Option<u64> {
    if !real_dylibs_present() {
        eprintln!("skipping {}: run `make dyld` to extract macOS dylibs", name);
        return None;
    }
    helpers::setup();
    let tmp = std::env::temp_dir().join(format!("mwemu_test_{}.bin", name));
    std::fs::write(&tmp, bytes).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());
    let _ = emu.run(None);
    Some(emu.processes.exit_status.expect("program did not exit"))
}

#[test]
fn macho64_fork_children_compute_and_are_reaped() {
    if let Some(status) = run_fixture("macho64_aarch64_fork", FORK) {
        assert_eq!(status, 0);
    }
}

#[test]
fn macho64_pthread_create_join() {
    if let Some(status) = run_fixture("macho64_aarch64_pthread", PTHREAD) {
        assert_eq!(status, 0);
    }
}

#[test]
fn macho64_fork_edge_cases() {
    if let Some(status) = run_fixture("macho64_aarch64_fork_edge", FORK_EDGE) {
        assert_eq!(status, 0);
    }
}
