/// Full end-to-end emulation of the host's real `/bin/ls` in `--libc` mode
/// (execute the genuine ld.so + libc, intercepting only at `syscall`).
///
/// This only makes sense on an x86_64 Linux host, where `/bin/ls` and the
/// system interpreter/libc actually exist. On any other host (macOS, Windows,
/// aarch64 Linux, or a box without `/bin/ls`) the test no-ops so the suite
/// stays green everywhere.
#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[ignore = "environment-dependent: emulates the host's real /bin/ls over its real glibc, so \
            success hinges on the exact glibc/coreutils build. Reliable on a tuned host (Arch), \
            but the CI runners' glibc diverges. Run on demand with --ignored."]
fn elf64lin_real_ls_full_emulation() {
    use crate::tests::helpers;
    use crate::*;
    helpers::setup();

    let ls = "/bin/ls";
    if !std::path::Path::new(ls).exists() {
        eprintln!("skipping: {} not present on host", ls);
        return;
    }
    // Make sure the on-disk binary is a 64-bit x86 ELF (ELF magic + EI_CLASS=2
    // + e_machine=EM_X86_64); skip if the host ships something else.
    let head = std::fs::read(ls).unwrap_or_default();
    let is_elf64_x64 = head.len() > 18
        && &head[0..4] == b"\x7fELF"
        && head[4] == 2 // ELFCLASS64
        && head[18] == 0x3e; // EM_X86_64
    if !is_elf64_x64 {
        eprintln!("skipping: {} is not an x86_64 ELF on this host", ls);
        return;
    }

    let mut emu = emu64();
    // Run the genuine libc/ld.so and intercept only at `syscall` (the Linux
    // counterpart of --ssdt). This is the default on Linux but set it
    // explicitly so the test is independent of any future default change.
    emu.cfg.linux_real_libc = true;
    // Guard against a runaway loop: a clean `ls` of a small directory settles
    // well under this many emulated instructions.
    emu.max_pos = Some(20_000_000);
    emu.load_code(ls);

    assert!(emu.cfg.arch.is_x64());
    assert!(emu.os.is_linux(), "expected the Linux loader path");

    let _ = emu.run(None);

    // The process must have reached the `exit`/`exit_group` syscall (which
    // calls `emu.stop()`), not hit the instruction cap.
    assert!(
        emu.max_pos.map(|cap| emu.pos < cap).unwrap_or(true),
        "ls did not terminate: hit the {:?}-instruction cap at pos {}",
        emu.max_pos,
        emu.pos
    );
    // `ls` of an accessible directory exits 0; the status lands in rdi at the
    // exit syscall. A non-zero code here means the emulated libc hit an error
    // path (e.g. a failed close()/write() on the std streams).
    assert_eq!(
        emu.regs().rdi,
        0,
        "ls should exit(0); got exit({})",
        emu.regs().rdi
    );
}
