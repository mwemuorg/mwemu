//! SSE scalar/packed move tests (x86-64), verified against the real instruction
//! semantics. These are self-contained (no external corpus): each test assembles
//! a tiny code stub, runs it, and checks the resulting XMM register / memory.
//!
//! The `movhps` load test is a regression guard for the bug where
//! `movhps xmm, m64` read only 32 bits (placing them at bits [127:96]) instead
//! of loading the full 64-bit qword into the high half — which corrupted
//! pointer-pair copies in glibc/coreutils (`make test_linux` crash).

use crate::*;

const SCRATCH: u64 = 0x800000;

fn run_code(code: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    // Scratch data region for the memory operands (created after load_code_bytes,
    // which calls init_cpu and resets maps).
    emu.maps
        .create_map(
            "scratch",
            SCRATCH,
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE,
        )
        .expect("scratch map");
    emu.run(None).unwrap();
    emu
}

/// `movhps xmm0, [rax]` must load the FULL 64-bit qword into the high half,
/// preserving the low half. Regression for the 32-bit-read bug.
#[test]
fn movhps_load_high_qword() {
    // mov rax, 0x800000
    // mov rbx, 0x1122334455667788
    // mov [rax], rbx              ; mem holds the qword to load into the high half
    // mov rcx, 0xCAFEBABEDEADBEEF
    // movq xmm0, rcx             ; xmm0 low half = sentinel
    // movhps xmm0, [rax]         ; xmm0 high half = mem qword
    // hlt
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x80, 0x00, // mov rax, 0x800000
        0x48, 0xbb, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, // mov rbx, 0x1122334455667788
        0x48, 0x89, 0x18, // mov [rax], rbx
        0x48, 0xb9, 0xef, 0xbe, 0xad, 0xde, 0xbe, 0xba, 0xfe,
        0xca, // mov rcx, 0xCAFEBABEDEADBEEF
        0x66, 0x48, 0x0f, 0x6e, 0xc1, // movq xmm0, rcx
        0x0f, 0x16, 0x00, // movhps xmm0, [rax]
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    let xmm0 = emu.regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0x1122334455667788_CAFEBABEDEADBEEFu128,
        "movhps must load the full 64-bit qword into the high half (got {:032x})",
        xmm0
    );
}

/// `movlps xmm0, [rax]` loads the low 64 bits from memory, preserving the high half.
#[test]
fn movlps_load_low_qword() {
    // mov rax, 0x800000
    // mov rbx, 0x99AABBCCDDEEFF00
    // mov [rax], rbx
    // mov rcx, 0x1111111122222222
    // movq xmm0, rcx        ; xmm0 = 0 : 0x1111111122222222
    // pslldq xmm0, 8        ; xmm0 = 0x1111111122222222 : 0   (move it to the high half)
    // movlps xmm0, [rax]    ; low half = mem qword, high half preserved
    // hlt
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x80, 0x00, // mov rax, 0x800000
        0x48, 0xbb, 0x00, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa,
        0x99, // mov rbx, 0x99AABBCCDDEEFF00
        0x48, 0x89, 0x18, // mov [rax], rbx
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x11, 0x11, 0x11,
        0x11, // mov rcx, 0x1111111122222222
        0x66, 0x48, 0x0f, 0x6e, 0xc1, // movq xmm0, rcx
        0x66, 0x0f, 0x73, 0xf8, 0x08, // pslldq xmm0, 8
        0x0f, 0x12, 0x00, // movlps xmm0, [rax]
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    let xmm0 = emu.regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0x1111111122222222_99AABBCCDDEEFF00u128,
        "movlps must load the low qword and preserve the high half (got {:032x})",
        xmm0
    );
}

/// `movups` 128-bit load + `movhps`/`movups` stores round-trip correctly.
#[test]
fn movups_and_movhps_store() {
    // mov rax, 0x800000
    // mov rbx, 0xAAAAAAAAAAAAAAAA ; low qword source
    // mov [rax+0x10], rbx
    // mov rbx, 0xBBBBBBBBBBBBBBBB ; high qword source
    // mov [rax+0x18], rbx
    // movups xmm0, [rax+0x10]     ; xmm0 = BBBB.. : AAAA..
    // movhps [rax], xmm0          ; store high qword (BBBB..) to [rax]
    // movups [rax+0x20], xmm0     ; store full 128 bits to [rax+0x20]
    // hlt
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x80, 0x00, // mov rax, 0x800000
        0x48, 0xbb, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, // mov rbx, 0xAAAA...
        0x48, 0x89, 0x58, 0x10, // mov [rax+0x10], rbx
        0x48, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, // mov rbx, 0xBBBB...
        0x48, 0x89, 0x58, 0x18, // mov [rax+0x18], rbx
        0x0f, 0x10, 0x40, 0x10, // movups xmm0, [rax+0x10]
        0x0f, 0x17, 0x00, // movhps [rax], xmm0
        0x0f, 0x11, 0x40, 0x20, // movups [rax+0x20], xmm0
        0xf4, // hlt
    ];
    let emu = run_code(&code);

    let xmm0 = emu.regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0xBBBBBBBBBBBBBBBB_AAAAAAAAAAAAAAAAu128,
        "movups load (got {:032x})",
        xmm0
    );
    // movhps store wrote the HIGH qword to [rax].
    assert_eq!(
        emu.maps.read_qword(SCRATCH).unwrap(),
        0xBBBBBBBBBBBBBBBB,
        "movhps store should write the high qword"
    );
    // movups store wrote the full 128 bits to [rax+0x20].
    assert_eq!(
        emu.maps.read_qword(SCRATCH + 0x20).unwrap(),
        0xAAAAAAAAAAAAAAAA
    );
    assert_eq!(
        emu.maps.read_qword(SCRATCH + 0x28).unwrap(),
        0xBBBBBBBBBBBBBBBB
    );
}

/// `movhpd xmm0, [rax]` (double variant) loads the full 64-bit qword into the
/// high half, preserving the low half. Used to `unimplemented!()`-panic when the
/// operand was misdetected as 32-bit.
#[test]
fn movhpd_load_high_qword() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x80, 0x00, // mov rax, 0x800000
        0x48, 0xbb, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, // mov rbx, 0x1122334455667788
        0x48, 0x89, 0x18, // mov [rax], rbx
        0x48, 0xb9, 0xef, 0xbe, 0xad, 0xde, 0xbe, 0xba, 0xfe,
        0xca, // mov rcx, 0xCAFEBABEDEADBEEF
        0x66, 0x48, 0x0f, 0x6e, 0xc1, // movq xmm0, rcx
        0x66, 0x0f, 0x16, 0x00, // movhpd xmm0, [rax]
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    let xmm0 = emu.regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0x1122334455667788_CAFEBABEDEADBEEFu128,
        "movhpd must load the full 64-bit qword into the high half (got {:032x})",
        xmm0
    );
}

/// `movlpd xmm0, [rax]` (double variant) loads the low 64 bits, preserving the high half.
#[test]
fn movlpd_load_low_qword() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x80, 0x00, // mov rax, 0x800000
        0x48, 0xbb, 0x00, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa,
        0x99, // mov rbx, 0x99AABBCCDDEEFF00
        0x48, 0x89, 0x18, // mov [rax], rbx
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x11, 0x11, 0x11,
        0x11, // mov rcx, 0x1111111122222222
        0x66, 0x48, 0x0f, 0x6e, 0xc1, // movq xmm0, rcx
        0x66, 0x0f, 0x73, 0xf8, 0x08, // pslldq xmm0, 8
        0x66, 0x0f, 0x12, 0x00, // movlpd xmm0, [rax]
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    let xmm0 = emu.regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0x1111111122222222_99AABBCCDDEEFF00u128,
        "movlpd must load the low qword and preserve the high half (got {:032x})",
        xmm0
    );
}

/// `movhpd [rax], xmm0` / `movlpd [rax], xmm0` store the high / low qword (64-bit).
#[test]
fn movhpd_movlpd_store() {
    // Build xmm0 = 0xBBBB.. : 0xAAAA.. via movups, then store halves.
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x80, 0x00, // mov rax, 0x800000
        0x48, 0xbb, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, // mov rbx, 0xAAAA...
        0x48, 0x89, 0x58, 0x10, // mov [rax+0x10], rbx
        0x48, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, // mov rbx, 0xBBBB...
        0x48, 0x89, 0x58, 0x18, // mov [rax+0x18], rbx
        0x0f, 0x10, 0x40, 0x10, // movups xmm0, [rax+0x10]
        0x66, 0x0f, 0x17, 0x00, // movhpd [rax], xmm0      -> high (BBBB..)
        0x66, 0x0f, 0x13, 0x40, 0x08, // movlpd [rax+8], xmm0 -> low (AAAA..)
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(
        emu.maps.read_qword(SCRATCH).unwrap(),
        0xBBBBBBBBBBBBBBBB,
        "movhpd store should write the high qword"
    );
    assert_eq!(
        emu.maps.read_qword(SCRATCH + 8).unwrap(),
        0xAAAAAAAAAAAAAAAA,
        "movlpd store should write the low qword"
    );
}
