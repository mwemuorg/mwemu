use crate::tests::helpers;
use crate::*;

/// Raw syscall hello world (no libc, no dylibs):
///   mov x0, #1; adrp x1, msg@PAGE; add x1, x1, msg@PAGEOFF; mov x2, #14;
///   mov x16, #4; svc #0x80; mov x0, #0; mov x16, #1; svc #0x80
/// Compiled with: cc -arch arm64 -nostdlib -static -e _start -o test hello_raw.s
const HELLO_RAW: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_hello_raw.bin");

/// Standard hello world using printf and libSystem.B.dylib:
///   int main() { printf("Hello, World!\n"); return 0; }
/// Compiled with: cc -arch arm64 -o test hello.c
const HELLO_LIBC: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_hello.bin");

#[test]
fn macho64_hello_raw_syscall() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_macho64_hello_raw.bin");
    std::fs::write(&tmp, HELLO_RAW).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_aarch64());
    let pc = emu.regs_aarch64().pc;
    assert!(pc >= 0x100000000, "entry 0x{:x} should be in __TEXT", pc);

    // Run until SVC or max instructions
    let mut hit_svc = false;
    for i in 0..20 {
        let pc_before = emu.regs_aarch64().pc;
        let ok = emu.step();
        assert!(ok, "step {} failed at pc=0x{:x}", i, pc_before);
        assert_ne!(emu.regs_aarch64().pc, 0, "pc should never fall to 0");
        // After executing SVC for write (x16=4), check x0/x1/x2
        // After executing SVC for exit (x16=1), stop
        if emu.regs_aarch64().x[16] == 1 {
            hit_svc = true;
            break;
        }
    }
    assert!(hit_svc, "should have reached exit syscall");
}

#[test]
fn macho64_hello_libc_load() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_macho64_hello_libc.bin");
    std::fs::write(&tmp, HELLO_LIBC).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_aarch64());
    let pc = emu.regs_aarch64().pc;
    assert!(pc >= 0x100000000, "entry 0x{:x} should be in __TEXT", pc);
    assert!(emu.macho64.is_some(), "Mach-O metadata should be loaded");

    let macho = emu.macho64.as_ref().unwrap();
    let printf_addr = macho
        .addr_to_symbol
        .iter()
        .find(|(_, sym)| sym.trim_start_matches('_').contains("printf"))
        .map(|(addr, sym)| (*addr, sym.clone()))
        .expect("expected a resolved printf symbol in the loaded dylib map");

    let printf_map = emu.maps.get_addr_name(printf_addr.0).unwrap_or("unmapped");
    assert!(
        printf_map.contains("libSystem.B"),
        "printf should resolve into libSystem.B, got map '{}' at 0x{:x} ({})",
        printf_map,
        printf_addr.0,
        printf_addr.1
    );

    // Step until the first intercepted API call. Success means we reach a
    // library call, break cleanly, and never fall off into pc=0x0.
    let mut saw_api_break = false;
    let mut last_pc = pc;
    for i in 0..100 {
        let pc_before = emu.regs_aarch64().pc;
        let ok = emu.step();
        assert!(ok, "step {} failed at pc=0x{:x}", i, pc_before);
        last_pc = emu.regs_aarch64().pc;
        assert_ne!(last_pc, 0, "pc should never fall to 0");

        if emu.force_break {
            saw_api_break = true;
            break;
        }
    }

    assert!(saw_api_break, "expected to intercept a libSystem API call");
    assert_ne!(last_pc, 0, "final pc should be a valid return address");
}
