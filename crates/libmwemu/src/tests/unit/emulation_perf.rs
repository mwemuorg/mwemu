//! Throughput regression tests (roughly equivalent to timing `cargo run --release` on a sample).
//!
//! Mirrors: `cargo run --release -- -f test/sc32win_donut.bin -vv`
//! We omit `-c` / console so the run does not block on stdin and timings stay stable.
//!
//! **Run the check** (threshold only enforced in optimized builds):
//! `cargo test -p libmwemu sc32win_donut_emulation_throughput --release -- --nocapture`
//!
//! **Tune** after measuring on a reference machine (printed on failure):
//! - `MWEMU_PERF_INSTRUCTION_BUDGET` — default `3_000_000`
//! - `MWEMU_PERF_MIN_IPS` — minimum emulated instructions per second (wall clock); default `150_000.0`

use crate::tests::helpers;
use crate::*;

use std::time::{Duration, Instant};

/// Default work units: large enough to average out noise, small enough for CI (~seconds on a fast box).
const DEFAULT_INSTRUCTION_BUDGET: u64 = 3_000_000;

/// Conservative floor (ins/s). Raise after profiling on your slowest CI runner so noise does not fail builds.
const DEFAULT_MIN_IPS: f64 = 150_000.0;

#[test]
fn sc32win_donut_emulation_throughput_regression_guard() {
    helpers::setup();

    if cfg!(debug_assertions) {
        eprintln!(
            "sc32win_donut_emulation_throughput_regression_guard: skipped in debug builds \
             (run `cargo test -p libmwemu sc32win_donut_emulation_throughput --release`)"
        );
        return;
    }

    let budget: u64 = std::env::var("MWEMU_PERF_INSTRUCTION_BUDGET")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_INSTRUCTION_BUDGET);

    let min_ips: f64 = std::env::var("MWEMU_PERF_MIN_IPS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MIN_IPS);

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    let sample = helpers::test_data_path("sc32win_donut.bin");
    emu.load_code(&sample);

    // `-vv` → verbose level 2 (assembly tracing; same cost profile as CLI)
    emu.set_verbose(2);

    let t0 = Instant::now();
    emu.run_to(budget).expect("emulation should complete");
    let elapsed = t0.elapsed().as_secs_f64();
    assert!(
        elapsed > 0.0,
        "elapsed time underflow; budget={} pos={}",
        budget,
        emu.pos
    );

    let ips = budget as f64 / elapsed;
    assert!(
        ips >= min_ips,
        "emulation throughput regression: {:.0} ins/s < {:.0} ins/s (budget={} in {:.3}s). \
         Raise MWEMU_PERF_MIN_IPS if CI is legitimately slower, or fix the performance regression. \
         Calibrate with: cargo test -p libmwemu sc32win_donut_emulation_throughput --release -- --nocapture",
        ips,
        min_ips,
        budget,
        elapsed
    );

    eprintln!(
        "perf: sc32win_donut budget={} elapsed={:.3}s => {:.0} ins/s (min {:.0})",
        budget, elapsed, ips, min_ips
    );
}

/// Wall-clock benchmark guard, meant to catch large slowdowns.
///
/// Mirrors roughly: `time echo q | cargo run -- -f test/sc32win_donut.bin -vv -c 10000001`
/// (we don't spawn the interactive console in unit tests; we run to the same position).
#[test]
fn benchmark32win_donut() {
    helpers::setup();

    // The 25s budget is intended for optimized runs; debug builds are too slow/noisy.
    if cfg!(debug_assertions) {
        eprintln!("benchmark: skipped in debug builds (run with --release)");
        return;
    }

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    // Avoid any interactive console path from blocking the test runner.
    emu.cfg.console = false;
    emu.cfg.console_enabled = false;
    emu.disable_ctrlc();

    let sample = helpers::test_data_path("sc32win_donut.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing {}",
        sample
    );
    emu.load_code(&sample);

    // `-vv` → verbose level 2 (assembly tracing).
    emu.set_verbose(2);

    // In unit tests the logger defaults to `error`, so `-vv` won't actually print instructions.
    // To keep this benchmark meaningful, we run a larger instruction budget (tunable via env var).
    let target_pos: u64 = std::env::var("MWEMU_BENCH_DONUT_TARGET_POS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20_000_001);
    let budget = Duration::from_secs(25);
    let t0 = Instant::now();
    emu.run_to(target_pos).unwrap_or_else(|e| {
        panic!(
            "benchmark run_to({}) failed: {} (pos={} rip=0x{:x})",
            target_pos,
            e,
            emu.pos,
            emu.regs().rip
        );
    });
    let elapsed = t0.elapsed();

    assert!(
        elapsed <= budget,
        "benchmark regression: run_to({}) took {:?} (> {:?}) (pos={} rip=0x{:x})",
        target_pos,
        elapsed,
        budget,
        emu.pos,
        emu.regs().rip
    );
}

/// Wall-clock benchmark guard for the 64-bit Enigma sample.
///
/// Similar spirit to timing the CLI on `test/exe64win_enigma.bin`, but executed directly in Rust.
#[test]
fn benchmark64with_enigma() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    emu.cfg.console = false;
    emu.cfg.console_enabled = false;
    emu.disable_ctrlc();

    let sample = helpers::test_data_path("exe64win_enigma.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing {}",
        sample
    );
    emu.load_code(&sample);

    // Keep the cost profile comparable to `-vv`.
    emu.set_verbose(2);

    // In unit tests the logger defaults to `error`, so `-vv` won't actually print instructions.
    // Use a smaller instruction target in debug so it still finishes quickly.
    let (target_pos, budget) = if cfg!(debug_assertions) {
        let target_pos: u64 = std::env::var("MWEMU_BENCH_ENIGMA_TARGET_POS_DEBUG")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1_000_000);
        (target_pos, Duration::from_secs(20))
    } else {
        let target_pos: u64 = std::env::var("MWEMU_BENCH_ENIGMA_TARGET_POS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(12_000_000);
        let max_secs: u64 = std::env::var("MWEMU_BENCH_ENIGMA_MAX_SECS_RELEASE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(if cfg!(windows) { 13 } else { 7 });
        (target_pos, Duration::from_secs(max_secs))
    };
    let t0 = Instant::now();
    emu.run_to(target_pos).unwrap_or_else(|e| {
        panic!(
            "benchmark64with_enigma run_to({}) failed: {} (pos={} rip=0x{:x})",
            target_pos,
            e,
            emu.pos,
            emu.regs().rip
        );
    });
    let elapsed = t0.elapsed();

    assert!(
        elapsed <= budget,
        "benchmark64with_enigma regression: run_to({}) took {:?} (> {:?}) (pos={} rip=0x{:x})",
        target_pos,
        elapsed,
        budget,
        emu.pos,
        emu.regs().rip
    );
}
