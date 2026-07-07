//! Self-contained instruction emulation via `load_code_bytes` (no stack, no
//! Windows env, no bundle). Covers arithmetic/flags and hostile-code paths that
//! must fault gracefully rather than panic.

use crate::emu64;

/// mov rax,5 ; mov rbx,7 ; add rax,rbx  →  rax == 12
#[test]
fn add_updates_register() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // mov rax, 5
        0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // mov rbx, 7
        0x48, 0x01, 0xd8, // add rax, rbx
    ];
    emu.load_code_bytes(&code);
    for _ in 0..3 {
        emu.step();
    }
    assert_eq!(emu.regs().rax, 12);
}

/// xor eax,eax must set ZF and clear rax — a common flag path.
#[test]
fn xor_self_sets_zero_flag() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // mov rax, 0x99
        0x31, 0xc0, // xor eax, eax
    ];
    emu.load_code_bytes(&code);
    emu.step();
    emu.step();
    assert_eq!(emu.regs().rax, 0);
    assert!(emu.flags().f_zf, "xor eax,eax must set ZF");
}

/// Emulating raw garbage bytes must not panic — `step()` reports whether it can
/// keep going, but never crashes the analysis.
#[test]
fn garbage_code_does_not_panic() {
    let mut emu = emu64();
    let junk = [0xff, 0xff, 0xff, 0xff, 0x00, 0xcc, 0xf1, 0x0f, 0x0b];
    emu.load_code_bytes(&junk);
    for _ in 0..8 {
        // may return false (stop) or execute; must not panic
        if !emu.step() {
            break;
        }
    }
}

/// div by zero (#DE) must be handled as a fault, not a Rust panic.
#[test]
fn divide_by_zero_does_not_panic() {
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xd2, // xor rdx, rdx
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // mov rax, 10
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // mov rbx, 0
        0x48, 0xf7, 0xf3, // div rbx
    ];
    emu.load_code_bytes(&code);
    for _ in 0..4 {
        if !emu.step() {
            break;
        }
    }
}
