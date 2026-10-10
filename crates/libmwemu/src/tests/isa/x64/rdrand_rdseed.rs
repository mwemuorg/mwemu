//! RDRAND / RDSEED (x86-64). The returned value is random, so these tests
//! assert the deterministic part of the contract: the instruction dispatches
//! (reaching `hlt` via `run().unwrap()` proves it is not the unimplemented
//! fallback) and reports success by setting CF=1.

use crate::*;

fn run_code(code: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    emu.run(None).unwrap();
    emu
}

#[test]
fn rdrand64_sets_carry() {
    // clc                ; CF = 0
    // rdrand rax         ; must set CF = 1 on success
    // hlt
    let code = [0xf8, 0x48, 0x0f, 0xc7, 0xf0, 0xf4];
    let emu = run_code(&code);
    assert!(emu.flag_cf(), "RDRAND must set CF=1 on success");
}

#[test]
fn rdseed64_sets_carry() {
    // clc                ; CF = 0
    // rdseed rax         ; must set CF = 1 on success
    // hlt
    let code = [0xf8, 0x48, 0x0f, 0xc7, 0xf8, 0xf4];
    let emu = run_code(&code);
    assert!(emu.flag_cf(), "RDSEED must set CF=1 on success");
}

#[test]
fn rdrand32_zero_extends() {
    // mov rax, 0xffffffffffffffff
    // rdrand eax         ; writing a 32-bit reg must zero the upper 32 bits
    // hlt
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // mov rax, -1
        0x0f, 0xc7, 0xf0, // rdrand eax
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(
        emu.regs().rax >> 32,
        0,
        "RDRAND eax must zero the upper 32 bits of rax"
    );
    assert!(emu.flag_cf(), "RDRAND must set CF=1 on success");
}
