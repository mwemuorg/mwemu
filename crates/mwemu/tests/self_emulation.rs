#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use libmwemu::emu64;

#[test]
#[ignore = "uses the host's dynamic loader and libc; run with the release CLI"]
fn cli_startup_reaches_instruction_floor() {
    const STARTUP_FLOOR: u64 = 900_000;
    const INSTRUCTION_LIMIT: u64 = 2_000_000;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();
    let mut emu = emu64();
    emu.cfg.linux_real_libc = true;
    emu.load_code(env!("CARGO_BIN_EXE_mwemu"))
        .expect("load the mwemu CLI as a Linux guest");

    let outcome = emu.run_to(INSTRUCTION_LIMIT);
    assert!(
        emu.pos >= STARTUP_FLOOR,
        "guest CLI executed {} instructions (minimum {STARTUP_FLOOR}), RIP={:#x}, outcome={outcome:?}",
        emu.pos,
        emu.regs().rip
    );
}
