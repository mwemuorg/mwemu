//! Differential tests for mul/imul (CF/OF) and bt/bsf/bsr (CF/ZF) against the
//! SDM reference model in `oracle.rs`. These set their flags inline in the
//! handlers, so we emulate the real instruction end-to-end (`load_code_bytes` +
//! `step`) and compare.
//!
//! Flag comparison honors the spec: mul/imul define only CF/OF (SF/ZF/PF/AF
//! undefined); bt defines only CF; bsf/bsr define only ZF (plus the result
//! index when the source is non-zero). div/idiv leave *all* flags undefined, so
//! we don't flag-test them.

use super::oracle;
use crate::emu::Emu;
use crate::emu64;

/// Load a single instruction, set up state, single-step, return the emu.
fn run1(code: &[u8], setup: impl FnOnce(&mut Emu)) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code);
    setup(&mut emu);
    emu.step();
    emu
}

fn cf(emu: &mut Emu) -> bool {
    emu.flags_mut().materialize_lazy();
    emu.flags().f_cf
}
fn of(emu: &mut Emu) -> bool {
    emu.flags_mut().materialize_lazy();
    emu.flags().f_of
}
fn zf(emu: &mut Emu) -> bool {
    emu.flags_mut().materialize_lazy();
    emu.flags().f_zf
}

const V8: &[u8] = &[0, 1, 2, 0x0f, 0x10, 0x7f, 0x80, 0x81, 0xff, 0xaa];

/// MUL/IMUL r/m8 with `al = a`, `bl = b`: compare AX and CF/OF.
fn mul8_test(op: &str, opcode: &[u8], oracle: fn(u8, u8) -> (u16, u64)) {
    for &a in V8 {
        for &b in V8 {
            let (ax_ref, rf) = oracle(a, b);
            let mut emu = run1(opcode, |e| {
                e.regs_mut().rax = a as u64;
                e.regs_mut().rbx = b as u64;
            });
            assert_eq!(
                (emu.regs().rax & 0xffff) as u16,
                ax_ref,
                "{op}({a:#x},{b:#x}) ax"
            );
            assert_eq!(cf(&mut emu), rf & oracle::CF != 0, "{op}({a:#x},{b:#x}) CF");
            assert_eq!(of(&mut emu), rf & oracle::OF != 0, "{op}({a:#x},{b:#x}) OF");
        }
    }
}

// ---- MUL r/m8 (F6 /4 = mul bl): al*bl -> ax, CF=OF = (ah != 0) ----

#[test]
fn mul8_matches_oracle() {
    mul8_test("mul8", &[0xf6, 0xe3], oracle::mul8);
}

// ---- IMUL r/m8 (F6 /5 = imul bl): al*bl signed -> ax ----

#[test]
fn imul8_matches_oracle() {
    mul8_test("imul8", &[0xf6, 0xeb], oracle::imul8);
}

// ---- BT r/m32, r32 (0F A3 /r = bt eax, ebx): CF = tested bit ----

const V32: &[u32] = &[
    0,
    1,
    0x8000_0000,
    0x7fff_ffff,
    0xffff_ffff,
    0xaaaa_aaaa,
    0x0000_ff00,
];

#[test]
fn bt32_matches_oracle() {
    for &v in V32 {
        for bit in [0u32, 1, 7, 15, 16, 31, 32, 33, 63] {
            let rf = oracle::bt(v as u64, bit as u64, 32);
            let mut emu = run1(&[0x0f, 0xa3, 0xd8], |e| {
                e.regs_mut().rax = v as u64;
                e.regs_mut().rbx = bit as u64;
            });
            assert_eq!(
                cf(&mut emu),
                rf & oracle::CF != 0,
                "bt32({v:#x}, bit={bit}) CF"
            );
        }
    }
}

/// BSF/BSR r32, r/m32 with `eax = 0xdeadbeef`, `ebx = v`: compare ZF, and the
/// index in eax when the source is non-zero.
fn bitscan_test(op: &str, opcode: &[u8], oracle: fn(u64) -> (Option<u64>, u64)) {
    for &v in V32 {
        let (idx_ref, rf) = oracle(v as u64);
        let mut emu = run1(opcode, |e| {
            e.regs_mut().rax = 0xdead_beef;
            e.regs_mut().rbx = v as u64;
        });
        assert_eq!(zf(&mut emu), rf & oracle::ZF != 0, "{op}({v:#x}) ZF");
        if let Some(idx) = idx_ref {
            assert_eq!(emu.regs().rax & 0xffff_ffff, idx, "{op}({v:#x}) index");
        }
    }
}

// ---- BSF r32, r/m32 (0F BC /r = bsf eax, ebx): ZF=(src==0), else eax=index ----

#[test]
fn bsf32_matches_oracle() {
    bitscan_test("bsf32", &[0x0f, 0xbc, 0xc3], oracle::bsf);
}

// ---- BSR r32, r/m32 (0F BD /r): ZF=(src==0), else eax=index of top set bit ----

#[test]
fn bsr32_matches_oracle() {
    bitscan_test("bsr32", &[0x0f, 0xbd, 0xc3], |v| oracle::bsr(v, 32));
}
