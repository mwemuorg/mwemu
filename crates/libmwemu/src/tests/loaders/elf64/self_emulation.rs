/// mwemu emulating itself. Loads the built mwemu CLI binary, runs it over the
/// real libc/ld path, and asserts it self-emulates at least 1M instructions
/// before hitting an unimplemented instruction/syscall.
///
/// This is a **coverage ratchet**: the number only grows as instruction/syscall
/// coverage improves, so bump `FLOOR` when it climbs. Wherever it stops is the
/// next thing to implement — a self-generating TODO.
///
/// Ignored by default: it depends on the exact host binary (build + the host's
/// glibc/ld), so it's a local validation, not a CI gate (CI runners differ).
/// Build first, then: `cargo test -p libmwemu self_emulation -- --ignored --nocapture`.
#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[ignore = "environment-dependent self-emulation ratchet; build mwemu first, then run with --ignored"]
fn self_emulation_reaches_1m_instructions() {
    use crate::tests::helpers;
    use crate::*;
    helpers::setup();

    // Locate the built mwemu CLI binary (release preferred, debug fallback).
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest.join("../../target/release/mwemu"),
        manifest.join("../../target/debug/mwemu"),
    ];
    let bin = match candidates.iter().find(|p| p.is_file()) {
        Some(p) => p.to_string_lossy().into_owned(),
        None => {
            eprintln!("skipping: no built mwemu binary — run `cargo build --release` first");
            return;
        }
    };

    let mut emu = emu64();
    emu.cfg.linux_real_libc = true; // run the real glibc/ld, intercept only at syscalls
    // Safety cap comfortably above the target so a runaway never hangs the suite.
    emu.max_pos = Some(2_000_000);
    emu.load_code(&bin);

    // Runs until the emulated mwemu exits, hits an unimplemented instruction/
    // syscall (returns Err), or the cap; `emu.pos` is how far it self-emulated.
    let _ = emu.run(None);

    // With no arguments the emulated mwemu now runs its full Rust startup
    // (runtime init, /proc/self/maps, getrandom, …) to a clean exit at ~957K
    // instructions — i.e. it self-hosts end to end with no missing instruction.
    // Floor sits just under that as a coverage ratchet: a regression that adds a
    // blocker before this point drops the count and trips the test. Bump the
    // floor if the number climbs (e.g. once it's driven with a real workload).
    const FLOOR: u64 = 900_000;
    assert!(
        emu.pos >= FLOOR,
        "mwemu self-emulated only {} instructions (need >= {}). It stopped at \
         rip=0x{:x} — that instruction/syscall is the next one to implement.",
        emu.pos,
        FLOOR,
        emu.regs().rip,
    );
}
