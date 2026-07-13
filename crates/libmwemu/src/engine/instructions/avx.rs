// Shared helpers for VEX-encoded AVX/AVX2 instructions (not an instruction
// handler). Most AVX vertical ops apply the same 128-bit lane operation to each
// 128-bit half of a 256-bit operand, so these helpers factor that out.

use crate::arch::x86::regs::U256;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn to_pair(v: U256) -> (u128, u128) {
    let mut b = [0u8; 32];
    v.to_little_endian(&mut b);
    (
        u128::from_le_bytes(b[0..16].try_into().unwrap()),
        u128::from_le_bytes(b[16..32].try_into().unwrap()),
    )
}

pub fn from_pair(lo: u128, hi: u128) -> U256 {
    let mut b = [0u8; 32];
    b[0..16].copy_from_slice(&lo.to_le_bytes());
    b[16..32].copy_from_slice(&hi.to_le_bytes());
    U256::from_little_endian(&b)
}

/// Apply `f` per `bits`-wide lane across a 128-bit value.
pub fn lanes<F: Fn(u128, u128) -> u128>(a: u128, b: u128, bits: u32, f: F) -> u128 {
    let mask: u128 = if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    };
    let mut r = 0u128;
    let mut s = 0u32;
    while s < 128 {
        r |= (f((a >> s) & mask, (b >> s) & mask) & mask) << s;
        s += bits;
    }
    r
}

/// AVX compare predicate (imm8[4:0]); bit 4 (signaling) does not change the
/// boolean result, so only the low 4 bits select the comparison.
pub fn cmp_pred_f32(x: f32, y: f32, imm: u8) -> bool {
    let unord = x.is_nan() || y.is_nan();
    match imm & 0xf {
        0 => x == y,
        1 => x < y,
        2 => x <= y,
        3 => unord,
        4 => x != y,
        5 => !(x < y),
        6 => !(x <= y),
        7 => !unord,
        8 => x == y || unord,
        9 => !(x >= y),
        10 => !(x > y),
        11 => false,
        12 => x != y && !unord,
        13 => x >= y,
        14 => x > y,
        _ => true,
    }
}
pub fn cmp_pred_f64(x: f64, y: f64, imm: u8) -> bool {
    let unord = x.is_nan() || y.is_nan();
    match imm & 0xf {
        0 => x == y,
        1 => x < y,
        2 => x <= y,
        3 => unord,
        4 => x != y,
        5 => !(x < y),
        6 => !(x <= y),
        7 => !unord,
        8 => x == y || unord,
        9 => !(x >= y),
        10 => !(x > y),
        11 => false,
        12 => x != y && !unord,
        13 => x >= y,
        14 => x > y,
        _ => true,
    }
}

/// Apply `f` per f32 lane across a 128-bit value.
pub fn fps<F: Fn(f32, f32) -> f32>(a: u128, b: u128, f: F) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let x = f32::from_bits(((a >> (i * 32)) & 0xffffffff) as u32);
        let y = f32::from_bits(((b >> (i * 32)) & 0xffffffff) as u32);
        r |= (f(x, y).to_bits() as u128) << (i * 32);
    }
    r
}

/// Apply `f` per f64 lane across a 128-bit value.
pub fn fpd<F: Fn(f64, f64) -> f64>(a: u128, b: u128, f: F) -> u128 {
    let mut r = 0u128;
    for i in 0..2 {
        let x = f64::from_bits(((a >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
        let y = f64::from_bits(((b >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
        r |= (f(x, y).to_bits() as u128) << (i * 64);
    }
    r
}

/// 3-operand VEX binary op: DEST = f(SRC1, SRC2) applied to the 128-bit form or
/// to each 128-bit half of the 256-bit form.
pub fn binop<F: Fn(u128, u128) -> u128>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            let b = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, f(a, b));
        }
        256 => {
            let (a0, a1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            let (b0, b1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 2, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(ins, 0, from_pair(f(a0, b0), f(a1, b1)));
        }
        _ => return false,
    }
    true
}

/// VEX packed shift: shift each `bits`-wide lane of SRC1 (op1) by the count from
/// op2 (an imm8, or the low 64 bits of an xmm). `kind`: 0=left logical, 1=right
/// logical, 2=right arithmetic. A count >= width produces 0 (or the sign fill).
pub fn shift(emu: &mut Emu, ins: &Instruction, bits: u32, kind: u8) -> bool {
    let count = if emu.get_operand_sz(ins, 2) == 128 {
        (emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0) & 0xffff_ffff_ffff_ffff) as u64
    } else {
        emu.get_operand_value(ins, 2, true).unwrap_or(0)
    };
    let mask: u128 = if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    };
    let per128 = |data: u128| -> u128 {
        let mut r = 0u128;
        let mut s = 0u32;
        while s < 128 {
            let lane = (data >> s) & mask;
            let out = if count >= bits as u64 {
                if kind == 2 && (lane >> (bits - 1)) & 1 == 1 {
                    mask
                } else {
                    0
                }
            } else {
                match kind {
                    0 => (lane << count) & mask,
                    1 => lane >> count,
                    _ => {
                        let se = ((lane << (128 - bits)) as i128) >> (128 - bits);
                        ((se >> count) as u128) & mask
                    }
                }
            };
            r |= out << s;
            s += bits;
        }
        r
    };
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let d = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, per128(d));
        }
        256 => {
            let (lo, hi) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(ins, 0, from_pair(per128(lo), per128(hi)));
        }
        _ => return false,
    }
    true
}

/// Interleave the low halves of `a` and `b` at `bits`-wide granularity.
pub fn unpack_lo(a: u128, b: u128, bits: u32) -> u128 {
    let n = 128 / bits;
    let mask: u128 = if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    };
    let mut r = 0u128;
    for i in 0..(n / 2) {
        r |= ((a >> (i * bits)) & mask) << (2 * i * bits);
        r |= ((b >> (i * bits)) & mask) << ((2 * i + 1) * bits);
    }
    r
}

/// Interleave the high halves of `a` and `b` at `bits`-wide granularity.
pub fn unpack_hi(a: u128, b: u128, bits: u32) -> u128 {
    let n = 128 / bits;
    let mask: u128 = if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    };
    let mut r = 0u128;
    for i in 0..(n / 2) {
        r |= ((a >> ((n / 2 + i) * bits)) & mask) << (2 * i * bits);
        r |= ((b >> ((n / 2 + i) * bits)) & mask) << ((2 * i + 1) * bits);
    }
    r
}

/// 3-operand VEX scalar-single op: DEST[31:0] = f(SRC1[31:0], SRC2[31:0]);
/// DEST[127:32] comes from SRC1. (For unary scalars like VSQRTSS, ignore the
/// first argument.)
pub fn scalar32<F: Fn(f32, f32) -> f32>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    let src1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let src2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let x = f32::from_bits(src1 as u32);
    let y = f32::from_bits(src2 as u32);
    let r = (src1 & !0xffff_ffffu128) | (f(x, y).to_bits() as u128);
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}

/// 3-operand VEX scalar-double op: DEST[63:0] = f(SRC1[63:0], SRC2[63:0]);
/// DEST[127:64] comes from SRC1.
pub fn scalar64<F: Fn(f64, f64) -> f64>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    let src1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let src2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let x = f64::from_bits(src1 as u64);
    let y = f64::from_bits(src2 as u64);
    let r = (src1 & !(0xffff_ffff_ffff_ffffu128)) | (f(x, y).to_bits() as u128);
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}

// FMA operand order: 0 = *132* (dest*op2 + op1), 1 = *213* (op1*dest + op2),
// 2 = *231* (op1*op2 + dest). `neg` negates the product, `sub` negates the
// addend. `mul_add` gives the single-rounding fused result.
fn fma32(a: f32, b: f32, c: f32, order: u8, neg: bool, sub: bool) -> f32 {
    let (x, y, add) = match order {
        0 => (a, c, b),
        1 => (b, a, c),
        _ => (b, c, a),
    };
    let x = if neg { -x } else { x };
    let add = if sub { -add } else { add };
    x.mul_add(y, add)
}
fn fma64(a: f64, b: f64, c: f64, order: u8, neg: bool, sub: bool) -> f64 {
    let (x, y, add) = match order {
        0 => (a, c, b),
        1 => (b, a, c),
        _ => (b, c, a),
    };
    let x = if neg { -x } else { x };
    let add = if sub { -add } else { add };
    x.mul_add(y, add)
}

/// Packed FMA over ps (dbl=false) or pd (dbl=true). `alt` enables the
/// add/sub alternation (Some(true)=even subtract/odd add, Some(false)=even add/
/// odd subtract); None uses the plain `sub` behaviour on every lane.
pub fn fma_packed(
    emu: &mut Emu,
    ins: &Instruction,
    dbl: bool,
    order: u8,
    neg: bool,
    sub: bool,
    alt: Option<bool>,
) -> bool {
    let per128 = |a: u128, b: u128, c: u128| -> u128 {
        let mut r = 0u128;
        if dbl {
            for i in 0..2u32 {
                let g = |v: u128| f64::from_bits(((v >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
                let s = match alt {
                    Some(even_sub) => (i % 2 == 0) == even_sub,
                    None => sub,
                };
                let v = fma64(g(a), g(b), g(c), order, neg, s);
                r |= (v.to_bits() as u128) << (i * 64);
            }
        } else {
            for i in 0..4u32 {
                let g = |v: u128| f32::from_bits(((v >> (i * 32)) & 0xffffffff) as u32);
                let s = match alt {
                    Some(even_sub) => (i % 2 == 0) == even_sub,
                    None => sub,
                };
                let v = fma32(g(a), g(b), g(c), order, neg, s);
                r |= (v.to_bits() as u128) << (i * 32);
            }
        }
        r
    };
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
            let b = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            let c = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, per128(a, b, c));
        }
        256 => {
            let (a0, a1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 0, true)
                    .unwrap_or(U256::from(0)),
            );
            let (b0, b1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            let (c0, c1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 2, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(
                ins,
                0,
                from_pair(per128(a0, b0, c0), per128(a1, b1, c1)),
            );
        }
        _ => return false,
    }
    true
}

/// Scalar FMA (ss/sd): low lane computed, DEST[127:low] copied from SRC1 (op1).
pub fn fma_scalar(
    emu: &mut Emu,
    ins: &Instruction,
    dbl: bool,
    order: u8,
    neg: bool,
    sub: bool,
) -> bool {
    let a = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let b = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let c = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let r = if dbl {
        let v = fma64(
            f64::from_bits(a as u64),
            f64::from_bits(b as u64),
            f64::from_bits(c as u64),
            order,
            neg,
            sub,
        );
        (b & !(0xffff_ffff_ffff_ffffu128)) | (v.to_bits() as u128)
    } else {
        let v = fma32(
            f32::from_bits(a as u32),
            f32::from_bits(b as u32),
            f32::from_bits(c as u32),
            order,
            neg,
            sub,
        );
        (b & !0xffff_ffffu128) | (v.to_bits() as u128)
    };
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}

/// VEX sign/zero extend (VPMOVSX*/VPMOVZX*): widen `sbits`-wide source lanes to
/// `dbits`-wide destination lanes. The destination width (128/256) sets how many
/// lanes are produced; the source low bits supply them.
pub fn pmovx(emu: &mut Emu, ins: &Instruction, sbits: u32, dbits: u32, sign: bool) -> bool {
    let dsz = emu.get_operand_sz(ins, 0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let count = dsz / dbits;
    let smask: u128 = (1u128 << sbits) - 1;
    let dmask: u128 = if dbits >= 128 {
        u128::MAX
    } else {
        (1u128 << dbits) - 1
    };
    let signbit: u128 = 1u128 << (sbits - 1);
    let mut lo = 0u128;
    let mut hi = 0u128;
    for i in 0..count {
        let raw = (src >> (i * sbits)) & smask;
        let ext = if sign && raw & signbit != 0 {
            raw | !smask
        } else {
            raw
        } & dmask;
        let pos = i * dbits;
        if pos < 128 {
            lo |= ext << pos;
        } else {
            hi |= ext << (pos - 128);
        }
    }
    match dsz {
        128 => emu.set_operand_xmm_value_128(ins, 0, lo),
        256 => emu.set_operand_ymm_value_256(ins, 0, from_pair(lo, hi)),
        _ => return false,
    }
    true
}

/// VEX broadcast: replicate the low `bits`-wide element of the source across the
/// whole destination (xmm or ymm).
pub fn broadcast(emu: &mut Emu, ins: &Instruction, bits: u32) -> bool {
    let mask: u128 = if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    };
    let val = if ins.op_kind(1) == iced_x86::OpKind::Memory {
        (emu.get_operand_value(ins, 1, true).unwrap_or(0) as u128) & mask
    } else {
        emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0) & mask
    };
    let mut per128 = 0u128;
    let mut s = 0u32;
    while s < 128 {
        per128 |= val << s;
        s += bits;
    }
    match emu.get_operand_sz(ins, 0) {
        128 => emu.set_operand_xmm_value_128(ins, 0, per128),
        256 => emu.set_operand_ymm_value_256(ins, 0, from_pair(per128, per128)),
        _ => return false,
    }
    true
}

/// 4-operand VEX binary op with imm8 (op3): DEST = f(SRC1, SRC2, imm8) per half.
pub fn binop_imm<F: Fn(u128, u128, u8) -> u128>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    let imm = emu.get_operand_value(ins, 3, true).unwrap_or(0) as u8;
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            let b = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, f(a, b, imm));
        }
        256 => {
            let (a0, a1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            let (b0, b1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 2, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(ins, 0, from_pair(f(a0, b0, imm), f(a1, b1, imm)));
        }
        _ => return false,
    }
    true
}

/// 3-operand VEX unary op with imm8 (op2): DEST = f(SRC, imm8) per half.
pub fn unop_imm<F: Fn(u128, u8) -> u128>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u8;
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, f(a, imm));
        }
        256 => {
            let (a0, a1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(ins, 0, from_pair(f(a0, imm), f(a1, imm)));
        }
        _ => return false,
    }
    true
}

/// 3-operand VEX op where DEST (op0) is also read (accumulate): DEST =
/// f(DEST, SRC1, SRC2) per 128-bit half.
pub fn ternop_acc<F: Fn(u128, u128, u128) -> u128>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
            let a = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            let b = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, f(d, a, b));
        }
        256 => {
            let (d0, d1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 0, true)
                    .unwrap_or(U256::from(0)),
            );
            let (a0, a1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            let (b0, b1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 2, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(ins, 0, from_pair(f(d0, a0, b0), f(d1, a1, b1)));
        }
        _ => return false,
    }
    true
}

/// 2-operand VEX unary op: DEST = f(SRC).
pub fn unop<F: Fn(u128) -> u128>(emu: &mut Emu, ins: &Instruction, f: F) -> bool {
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
            emu.set_operand_xmm_value_128(ins, 0, f(a));
        }
        256 => {
            let (a0, a1) = to_pair(
                emu.get_operand_ymm_value_256(ins, 1, true)
                    .unwrap_or(U256::from(0)),
            );
            emu.set_operand_ymm_value_256(ins, 0, from_pair(f(a0), f(a1)));
        }
        _ => return false,
    }
    true
}

/// IEEE-754 half (f16) bit pattern -> f32 bit pattern (exact).
pub fn f16_to_f32(h: u16) -> u32 {
    let sign = (h as u32 & 0x8000) << 16;
    let exp = (h >> 10) & 0x1f;
    let mant = (h & 0x3ff) as u32;
    if exp == 0 {
        if mant == 0 {
            sign
        } else {
            let mut e = 0i32;
            let mut m = mant;
            while m & 0x400 == 0 {
                m <<= 1;
                e += 1;
            }
            sign | (((127 - 15 - e) as u32) << 23) | ((m & 0x3ff) << 13)
        }
    } else if exp == 0x1f {
        sign | 0x7f80_0000 | (mant << 13)
    } else {
        sign | (((exp as u32) + 127 - 15) << 23) | (mant << 13)
    }
}

/// f32 bit pattern -> IEEE-754 half (f16) with round-to-nearest-even.
pub fn f32_to_f16(x: u32) -> u16 {
    let sign = ((x >> 16) & 0x8000) as u16;
    let e = ((x >> 23) & 0xff) as i32 - 112;
    let m = x & 0x7fffff;
    if ((x >> 23) & 0xff) == 0xff {
        return sign | 0x7c00 | if m != 0 { 0x200 } else { 0 } | ((m >> 13) as u16 & 0x1ff);
    }
    if e >= 0x1f {
        return sign | 0x7c00;
    }
    if e <= 0 {
        if e < -10 {
            return sign;
        }
        let m = m | 0x800000;
        let s = (14 - e) as u32;
        let r = m >> s;
        let rem = m & ((1 << s) - 1);
        let half = 1u32 << (s - 1);
        let round = if rem > half || (rem == half && (r & 1) == 1) {
            1
        } else {
            0
        };
        return sign | ((r + round) as u16);
    }
    let r = m >> 13;
    let rem = m & 0x1fff;
    let round = if rem > 0x1000 || (rem == 0x1000 && (r & 1) == 1) {
        1
    } else {
        0
    };
    let mut mm = r + round;
    let mut ee = e;
    if mm & 0x400 != 0 {
        mm = 0;
        ee += 1;
        if ee >= 0x1f {
            return sign | 0x7c00;
        }
    }
    sign | ((ee as u16) << 10) | (mm as u16)
}
