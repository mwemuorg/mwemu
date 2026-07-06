use crate::tests::helpers;
use crate::*;

/// Raw macOS x86_64 syscall hello world (no libc, no dylibs).
/// Built from `fixtures/macho64_x86_64_hello_raw.s`.
const HELLO_RAW_X64: &[u8] = include_bytes!("../../fixtures/macho64_x86_64_hello_raw.bin");

/// macOS x86_64 libc-linked hello world using `puts`.
/// Built from `fixtures/macho64_x86_64_hello.c`.
const HELLO_LIBC_X64: &[u8] = include_bytes!("../../fixtures/macho64_x86_64_hello.bin");

#[test]
fn macho64_x64_hello_raw_syscall() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_macho64_x64_hello_raw.bin");
    std::fs::write(&tmp, HELLO_RAW_X64).unwrap();

    let mut emu = emu64();
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_x64());
    assert!(emu.os.is_macos(), "expected macOS loader path");

    let entry = emu.regs().rip;
    assert!(
        entry >= 0x100000000,
        "entry 0x{:x} should be in __TEXT",
        entry
    );

    let stopped_at = emu
        .run(None)
        .expect("raw macOS x86_64 syscall hello should exit cleanly");

    assert_ne!(stopped_at, 0, "run() should not stop at pc=0");
    assert!(
        emu.pos > 0,
        "emulation should execute at least one instruction"
    );
    assert_eq!(emu.regs().rax, 0x2000001, "final syscall should be exit");
    assert_eq!(emu.regs().rdi, 0, "exit status should be zero");
}

#[test]
fn macho64_x64_hello_libc_intercepts_puts() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_macho64_x64_hello_libc.bin");
    std::fs::write(&tmp, HELLO_LIBC_X64).unwrap();

    let mut emu = emu64();
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_x64());
    assert!(emu.os.is_macos(), "expected macOS loader path");
    assert!(emu.macho64.is_some(), "Mach-O metadata should be loaded");

    let macho = emu.macho64.as_ref().unwrap();
    let puts_addr = macho
        .addr_to_symbol
        .iter()
        .find(|(_, sym)| sym.trim_start_matches('_') == "puts")
        .map(|(addr, sym)| (*addr, sym.clone()))
        .expect("expected a resolved puts symbol in the loaded dylib map");

    let puts_map = emu.maps.get_addr_name(puts_addr.0).unwrap_or("unmapped");
    assert!(
        puts_map.contains("libSystem.B"),
        "puts should resolve into libSystem.B, got map '{}' at 0x{:x} ({})",
        puts_map,
        puts_addr.0,
        puts_addr.1
    );

    let mut saw_api_break = false;
    for i in 0..128 {
        let rip_before = emu.regs().rip;
        let ok = emu.step();
        assert!(ok, "step {} failed at rip=0x{:x}", i, rip_before);
        assert_ne!(emu.regs().rip, 0, "rip should never fall to 0");

        if emu.force_break {
            saw_api_break = true;
            break;
        }
    }

    assert!(saw_api_break, "expected to intercept a libSystem API call");
    assert_eq!(
        emu.regs().rax,
        0,
        "puts should return 0 through the API shim"
    );
    assert_ne!(
        emu.regs().rip,
        0,
        "final rip should be a valid return address"
    );
}
