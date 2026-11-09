//! Host-independent IEEE-754 helpers reproducing the x86 SSE/AVX NaN rules
//! (Intel SDM Vol. 1, §4.8.3.5 and Table 4-7).
//!
//! The arithmetic handlers used to feed the raw operands to the host FPU.
//! Hosts disagree on the NaN they *generate* for an invalid operation (x86
//! returns the negative "real indefinite" QNaN, AArch64 the positive default
//! NaN) and LLVM promises nothing about payload propagation, so the same
//! guest instruction gave different bits on an x86 host and on Apple Silicon.
//! Here a NaN input never reaches the host arithmetic and a NaN produced from
//! non-NaN inputs is canonicalised, so the result is the x86 one everywhere.
//!
//! Rules (same for scalar and packed forms):
//! - one NaN operand: that operand, quieted;
//! - two NaN operands: the first (destination / SRC1) operand, quieted;
//! - no NaN operand but an invalid operation (inf - inf, 0 / 0, 0 * inf,
//!   sqrt of a negative): the real indefinite QNaN.
//!
//! MIN/MAX follow a different rule (return SRC2 untouched when either input
//! is a NaN) and must not use these helpers.

/// x86 "real indefinite" single-precision QNaN.
pub const F32_INDEFINITE: u32 = 0xffc0_0000;
/// x86 "real indefinite" double-precision QNaN.
pub const F64_INDEFINITE: u64 = 0xfff8_0000_0000_0000;

const F32_QUIET_BIT: u32 = 0x0040_0000;
const F64_QUIET_BIT: u64 = 0x0008_0000_0000_0000;

/// Convert an SNaN into the corresponding QNaN (no-op for any other value).
#[inline]
pub fn quiet_f32(x: f32) -> f32 {
    f32::from_bits(x.to_bits() | F32_QUIET_BIT)
}

/// Convert an SNaN into the corresponding QNaN (no-op for any other value).
#[inline]
pub fn quiet_f64(x: f64) -> f64 {
    f64::from_bits(x.to_bits() | F64_QUIET_BIT)
}

/// Two-operand f32 operation with the x86 NaN rules; `a` is the first operand.
#[inline]
pub fn binop_f32(a: f32, b: f32, f: impl FnOnce(f32, f32) -> f32) -> f32 {
    if a.is_nan() {
        return quiet_f32(a);
    }
    if b.is_nan() {
        return quiet_f32(b);
    }
    let r = f(a, b);
    if r.is_nan() {
        f32::from_bits(F32_INDEFINITE)
    } else {
        r
    }
}

/// Two-operand f64 operation with the x86 NaN rules; `a` is the first operand.
#[inline]
pub fn binop_f64(a: f64, b: f64, f: impl FnOnce(f64, f64) -> f64) -> f64 {
    if a.is_nan() {
        return quiet_f64(a);
    }
    if b.is_nan() {
        return quiet_f64(b);
    }
    let r = f(a, b);
    if r.is_nan() {
        f64::from_bits(F64_INDEFINITE)
    } else {
        r
    }
}

/// One-operand f32 operation with the x86 NaN rules.
#[inline]
pub fn unop_f32(a: f32, f: impl FnOnce(f32) -> f32) -> f32 {
    if a.is_nan() {
        return quiet_f32(a);
    }
    let r = f(a);
    if r.is_nan() {
        f32::from_bits(F32_INDEFINITE)
    } else {
        r
    }
}

/// One-operand f64 operation with the x86 NaN rules.
#[inline]
pub fn unop_f64(a: f64, f: impl FnOnce(f64) -> f64) -> f64 {
    if a.is_nan() {
        return quiet_f64(a);
    }
    let r = f(a);
    if r.is_nan() {
        f64::from_bits(F64_INDEFINITE)
    } else {
        r
    }
}

#[inline]
pub fn add_f32(a: f32, b: f32) -> f32 {
    binop_f32(a, b, |x, y| x + y)
}
#[inline]
pub fn sub_f32(a: f32, b: f32) -> f32 {
    binop_f32(a, b, |x, y| x - y)
}
#[inline]
pub fn mul_f32(a: f32, b: f32) -> f32 {
    binop_f32(a, b, |x, y| x * y)
}
#[inline]
pub fn div_f32(a: f32, b: f32) -> f32 {
    binop_f32(a, b, |x, y| x / y)
}
#[inline]
pub fn sqrt_f32(a: f32) -> f32 {
    unop_f32(a, f32::sqrt)
}
#[inline]
pub fn add_f64(a: f64, b: f64) -> f64 {
    binop_f64(a, b, |x, y| x + y)
}
#[inline]
pub fn sub_f64(a: f64, b: f64) -> f64 {
    binop_f64(a, b, |x, y| x - y)
}
#[inline]
pub fn mul_f64(a: f64, b: f64) -> f64 {
    binop_f64(a, b, |x, y| x * y)
}
#[inline]
pub fn div_f64(a: f64, b: f64) -> f64 {
    binop_f64(a, b, |x, y| x / y)
}
#[inline]
pub fn sqrt_f64(a: f64) -> f64 {
    unop_f64(a, f64::sqrt)
}
