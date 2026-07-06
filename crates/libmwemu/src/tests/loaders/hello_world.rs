//! Hello-world parity tests across all (os, arch) combos we care about.
//!
//! These exist to sniff out loader / ISA / OS-surface gaps. Each test loads a
//! tiny C `printf("hello world")` binary built by `examples/hello-world/Makefile`,
//! takes a bounded number of steps, and asserts the loader picked the right
//! arch. Tests for combos the emulator does not yet support are `#[ignore]`d
//! with a comment pointing at the gap so future work can flip them on.
//!
//! Source: examples/hello-world/main.c
//! Build:  make -C examples/hello-world all  (then move into tests/fixtures/)

use crate::tests::helpers;
use crate::*;

const HELLO_LINUX_X86: &[u8] = include_bytes!("../fixtures/hello_linux_x86");
const HELLO_LINUX_X64: &[u8] = include_bytes!("../fixtures/hello_linux_x64");
const HELLO_LINUX_ARM64: &[u8] = include_bytes!("../fixtures/hello_linux_arm64");
const HELLO_MAC_ARM64: &[u8] = include_bytes!("../fixtures/hello_mac_arm64");
const HELLO_MAC_X64: &[u8] = include_bytes!("../fixtures/hello_mac_x64");
const HELLO_WIN_X86: &[u8] = include_bytes!("../fixtures/hello_win_x86.exe");
const HELLO_WIN_X64: &[u8] = include_bytes!("../fixtures/hello_win_x64.exe");
const HELLO_WIN_ARM64: &[u8] = include_bytes!("../fixtures/hello_win_arm64.exe");

const MAX_STEPS: usize = 64;

fn write_tmp(name: &str, bytes: &[u8]) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

/// Dynamic ELF32 hello world -- loads and steps without null-pointer write.
#[test]
fn hello_linux_x86() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_linux_x86", HELLO_LINUX_X86);

    let mut emu = emu32();
    emu.load_code(path.to_str().unwrap());

    assert!(
        matches!(emu.cfg.arch, crate::arch::Arch::X86),
        "expected ELF32 x86 dispatch, got {:?}",
        emu.cfg.arch
    );
    let entry = emu.regs().rip;
    assert!(entry != 0, "entry point should be set");

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}

/// Dynamic ELF64 x86_64 hello world -- loads and steps with stack layout.
#[test]
fn hello_linux_x64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_linux_x64", HELLO_LINUX_X64);

    let mut emu = emu64();
    emu.load_code(path.to_str().unwrap());

    assert!(emu.cfg.arch.is_x64(), "expected ELF64 x86_64 dispatch");
    let entry = emu.regs().rip;
    assert!(entry != 0, "entry point should be set");

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}

/// Dynamic ELF64 AArch64 hello world — loads and steps correctly.
#[test]
fn hello_linux_arm64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_linux_arm64", HELLO_LINUX_ARM64);

    let mut emu = emu_aarch64();
    emu.load_code(path.to_str().unwrap());

    assert!(emu.cfg.arch.is_aarch64(), "expected ELF64 aarch64 dispatch");
    let pc = emu.regs_aarch64().pc;
    assert!(pc != 0, "pc should be set by loader");

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}

#[test]
fn hello_mac_arm64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_mac_arm64", HELLO_MAC_ARM64);

    let mut emu = emu_aarch64();
    emu.load_code(path.to_str().unwrap());

    assert!(
        emu.cfg.arch.is_aarch64(),
        "expected Mach-O aarch64 dispatch"
    );
    let pc = emu.regs_aarch64().pc;
    assert!(pc >= 0x100000000, "entry 0x{:x} should be in __TEXT", pc);

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}

/// Mach-O x86_64 hello world — loads and detects correct arch.
#[test]
fn hello_mac_x64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_mac_x64", HELLO_MAC_X64);

    let mut emu = emu64();
    emu.load_code(path.to_str().unwrap());

    assert!(
        emu.cfg.arch.is_x64(),
        "expected Mach-O x86_64 dispatch, got {:?}",
        emu.cfg.arch
    );
}

/// Windows PE32 x86 hello world — loads and detects correct arch.
#[test]
fn hello_win_x86() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_x86.exe", HELLO_WIN_X86);

    let mut emu = emu32();
    emu.load_code(path.to_str().unwrap());

    assert!(
        matches!(emu.cfg.arch, crate::arch::Arch::X86),
        "expected PE32 x86 dispatch, got {:?}",
        emu.cfg.arch
    );
    let entry = emu.regs().rip;
    assert!(entry != 0, "entry point should be set");

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}

/// Windows x86_64 PE hello world — loads and detects correct arch.
#[test]
fn hello_win_x64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_x64.exe", HELLO_WIN_X64);

    let mut emu = emu64();
    emu.load_code(path.to_str().unwrap());

    assert!(emu.cfg.arch.is_x64(), "expected PE64 x86_64 dispatch");
    let entry = emu.regs().rip;
    assert!(entry != 0, "entry point should be set");

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}

/// Windows ARM64 PE hello world — loads, detects arch, and steps.
/// Requires real ARM64 DLLs in maps/windows/aarch64/ (kernelbase.dll, kernel32.dll, ntdll.dll).
#[test]
#[ignore = "parity gap: maps/windows/aarch64/ has no DLLs yet"]
fn hello_win_arm64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_arm64.exe", HELLO_WIN_ARM64);

    let mut emu = emu_aarch64();
    emu.load_code(path.to_str().unwrap());

    assert!(
        emu.cfg.arch.is_aarch64(),
        "expected PE aarch64 dispatch, got {:?}",
        emu.cfg.arch
    );
    let pc = emu.pc();
    assert!(pc != 0, "entry point should be set");

    for _ in 0..MAX_STEPS {
        if !emu.step() {
            break;
        }
    }
}
