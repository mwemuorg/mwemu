//! Regression tests for x86 semantic bugs found by the `mwemu-x86-test` harness
//! (replaying hardware-oracle x86Tester vectors and diffing the CPU state).
//!
//! Each test assembles a tiny stub, runs it to `hlt`, and checks the exact value
//! the silicon produces. They are self-contained (no external corpus).

use crate::*;

fn run_code(code: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    emu.run(None).unwrap();
    emu
}

/// `movq xmm0, xmm0` (F3 0F 7E) must zero bits [127:64] of the destination, not
/// leave them untouched.
#[test]
fn movq_xmm_zeroes_upper_half() {
    let code = [
        0x66, 0x0f, 0x76, 0xc0, // pcmpeqd xmm0, xmm0  -> all ones
        0xf3, 0x0f, 0x7e, 0xc0, // movq    xmm0, xmm0  -> keep low 64, zero high 64
        0xf4, // hlt
    ];
    let xmm0 = run_code(&code).regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0x0000_0000_0000_0000_ffff_ffff_ffff_ffffu128,
        "movq must zero the upper 64 bits (got {xmm0:032x})"
    );
}

/// POPCNT must set ZF when the source is zero and clear the other status flags;
/// the buggy handler left the flags untouched.
#[test]
fn popcnt_sets_zf_on_zero() {
    let code = [
        0x31, 0xc0, // xor    eax, eax
        0xf3, 0x0f, 0xb8, 0xc0, // popcnt eax, eax  -> 0, ZF=1
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(emu.regs().get_eax(), 0, "popcnt of 0 is 0");
    assert!(emu.flags_snapshot().f_zf, "popcnt of 0 must set ZF");
    assert!(!emu.flags_snapshot().f_cf, "popcnt must clear CF");
}

/// LZCNT must count leading zeros at the operand width: LZCNT of a 16-bit zero
/// is 16, not the 64 that u64::leading_zeros would give.
#[test]
fn lzcnt_respects_operand_size() {
    let code = [
        0x31, 0xc0, // xor   eax, eax
        0xf3, 0x66, 0x0f, 0xbd, 0xc0, // lzcnt ax, ax  -> 16
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(emu.regs().get_ax(), 16, "16-bit LZCNT of 0 must be 16");
    assert!(emu.flags_snapshot().f_cf, "LZCNT of 0 must set CF");
}

/// CMPXCHG r/m8, r8 on a mismatch must load the destination into AL only,
/// preserving the rest of RAX — the buggy handler compared against the full RAX
/// and zeroed it.
#[test]
fn cmpxchg8_preserves_rax_upper() {
    let code = [
        0x48, 0xb8, 0x11, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
        0xcc, // mov rax, 0xcccccccccccccc11
        0xb3, 0x99, // mov bl, 0x99
        0xb1, 0x22, // mov cl, 0x22
        0x0f, 0xb0, 0xcb, // cmpxchg bl, cl   ; AL(0x11) != bl(0x99) -> AL = 0x99
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(
        emu.regs().rax,
        0xcccccccccccccc99,
        "cmpxchg must only overwrite AL, keeping RAX[63:8]"
    );
    assert!(!emu.flags_snapshot().f_zf, "mismatch must clear ZF");
}

/// IDIV is a signed division: -10 / 3 = -3 remainder -1, so AL = 0xFD, AH = 0xFF.
/// The buggy handler did an unsigned divide.
#[test]
fn idiv8_is_signed() {
    let code = [
        0x66, 0xb8, 0xf6, 0xff, // mov ax, 0xfff6  ; -10
        0xb1, 0x03, // mov cl, 3
        0xf6, 0xf9, // idiv cl
        0xf4, // hlt
    ];
    let emu = run_code(&code);
    assert_eq!(emu.regs().get_al(), 0xfd, "quotient -3");
    assert_eq!(emu.regs().get_ah(), 0xff, "remainder -1");
}

/// PADDW adds every 16-bit lane independently: 0xffff + 0xffff = 0xfffe in all
/// eight lanes. The buggy handler botched the top lane via a wrong mask.
#[test]
fn paddw_all_lanes() {
    let code = [
        0x66, 0x0f, 0x76, 0xc0, // pcmpeqd xmm0, xmm0 -> all ones
        0x66, 0x0f, 0xfd, 0xc0, // paddw   xmm0, xmm0
        0xf4, // hlt
    ];
    let xmm0 = run_code(&code).regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0xfffe_fffe_fffe_fffe_fffe_fffe_fffe_fffeu128,
        "every word must be 0xfffe (got {xmm0:032x})"
    );
}

/// PSUBW must keep each lane's borrow within that 16-bit lane: 0 - 0xffff =
/// 0x0001 per word. The buggy handler subtracted over the whole u128, so a
/// borrow flooded the upper lanes with ones.
#[test]
fn psubw_lane_isolation() {
    let code = [
        0x66, 0x0f, 0xef, 0xc0, // pxor    xmm0, xmm0  -> 0
        0x66, 0x0f, 0x76, 0xc9, // pcmpeqd xmm1, xmm1  -> all ones
        0x66, 0x0f, 0xf9, 0xc1, // psubw   xmm0, xmm1
        0xf4, // hlt
    ];
    let xmm0 = run_code(&code).regs().get_xmm_by_name("xmm0");
    assert_eq!(
        xmm0, 0x0001_0001_0001_0001_0001_0001_0001_0001u128,
        "each word must be 0x0001 (got {xmm0:032x})"
    );
}

/// CVTSI2SS converts the integer into bits [31:0] and leaves [127:32] intact.
/// The buggy handler zeroed the upper bits and skipped the conversion entirely.
#[test]
fn cvtsi2ss_preserves_upper_and_converts() {
    let code = [
        0x66, 0x0f, 0x76, 0xc0, // pcmpeqd  xmm0, xmm0 -> all ones
        0xb8, 0x04, 0x00, 0x00, 0x00, // mov eax, 4
        0xf3, 0x0f, 0x2a, 0xc0, // cvtsi2ss xmm0, eax  -> 4.0f in [31:0]
        0xf4, // hlt
    ];
    let xmm0 = run_code(&code).regs().get_xmm_by_name("xmm0");
    // 4.0f == 0x40800000; [127:32] preserved as ones.
    assert_eq!(
        xmm0, 0xffff_ffff_ffff_ffff_ffff_ffff_4080_0000u128,
        "cvtsi2ss must convert to f32 and preserve [127:32] (got {xmm0:032x})"
    );
}

/// UCOMISS of a NaN against anything is unordered: ZF, PF and CF are all set.
/// The buggy handler set only PF.
#[test]
fn ucomiss_unordered_sets_zf_pf_cf() {
    let code = [
        0xb8, 0x00, 0x00, 0xc0, 0x7f, // mov  eax, 0x7fc00000  ; qNaN
        0x66, 0x0f, 0x6e, 0xc0, // movd xmm0, eax
        0x0f, 0x2e, 0xc0, // ucomiss xmm0, xmm0
        0xf4, // hlt
    ];
    let f = run_code(&code).flags_snapshot();
    assert!(
        f.f_zf && f.f_pf && f.f_cf,
        "unordered compare sets ZF, PF and CF"
    );
}
