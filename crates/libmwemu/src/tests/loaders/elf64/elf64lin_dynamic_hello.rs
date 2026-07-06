use std::sync::atomic::Ordering;

use crate::tests::helpers;
use crate::*;

/// Dynamic Linux x86_64 hello world using `puts`, built from
/// `fixtures/elf64_x86_64_linux_hello.c`.
const ELF64_LINUX_X64_HELLO: &[u8] = include_bytes!("../../fixtures/elf64_x86_64_linux_hello.bin");

#[test]
fn elf64lin_dynamic_hello_puts_bridge() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_elf64_x86_64_linux_hello.bin");
    std::fs::write(&tmp, ELF64_LINUX_X64_HELLO).unwrap();

    let mut emu = emu64();
    // This test validates the in-Rust libc-hook fallback path specifically, so
    // disable the default real-ld.so/libc emulation here.
    emu.cfg.linux_real_libc = false;
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_x64());
    assert!(emu.os.is_linux(), "expected Linux loader path");
    assert!(emu.elf64.is_some(), "ELF metadata should be loaded");

    let elf = emu.elf64.as_ref().unwrap();
    assert!(
        elf.needed_libs.iter().any(|lib| lib.contains("libc.so.6")),
        "expected libc.so.6 in DT_NEEDED, got {:?}",
        elf.needed_libs
    );

    let start_main_addr = *elf
        .sym_to_addr
        .get("__libc_start_main")
        .expect("expected __libc_start_main to resolve into the linux stub libc");
    let puts_addr = *elf
        .sym_to_addr
        .get("puts")
        .expect("expected puts to resolve into the linux stub libc");

    let start_main_map = emu
        .maps
        .get_addr_name(start_main_addr)
        .unwrap_or("unmapped");
    let puts_map = emu.maps.get_addr_name(puts_addr).unwrap_or("unmapped");
    assert!(
        start_main_map.contains("libc.so.6"),
        "__libc_start_main should map into libc.so.6, got '{}'",
        start_main_map
    );
    assert!(
        puts_map.contains("libc.so.6"),
        "puts should map into libc.so.6, got '{}'",
        puts_map
    );

    let mut saw_unix_api_break = false;
    for i in 0..32 {
        let rip_before = emu.regs().rip;
        let ok = emu.step();
        assert!(ok, "step {} failed at rip=0x{:x}", i, rip_before);
        assert_ne!(emu.regs().rip, 0, "rip should never fall to 0");

        if emu.force_break {
            saw_unix_api_break = true;
            break;
        }
    }

    assert!(
        saw_unix_api_break,
        "expected to intercept the Linux x86_64 startup/libc path"
    );
    assert_eq!(
        emu.is_running.load(Ordering::Relaxed),
        0,
        "the process should be stopped after __libc_start_main drives main -> exit"
    );
    assert_eq!(
        emu.regs().rdi,
        7,
        "main return value should be forwarded into exit(status)"
    );
}
