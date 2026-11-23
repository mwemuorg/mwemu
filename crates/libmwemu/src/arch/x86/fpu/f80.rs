use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

pub const FPU_80_BITS_MAX: u128 = (1u128 << 80) - 1;
pub const REAL_INDEFINITE: u128 = 0xffff_c000_0000_0000_0000 & FPU_80_BITS_MAX;
pub const QNAN: u128 = 0x7fff_c000_0000_0000_0000 & FPU_80_BITS_MAX;
pub const F80_PI: u128 = 0x4000c90fdaa22168c234 & FPU_80_BITS_MAX;
pub const SIGN_MASK: u128 = 1 << 79;
pub const EXP_MASK: u128 = 0x7FFF;
pub const EXP_MASK2: u128 = 0x7FFF << 64;
pub const MANTISSA_MASK: u128 = (1u128 << 64) - 1;
pub const MANTISSA_MASK_NOINT: u128 = 0x7FFF_FFFF_FFFF_FFFF;
pub const INT_BIT_MASK: u128 = 1 << 63;
pub const INT_BIT_MASK64: u64 = 1 << 63;
pub const BCD_SIGN_POSITIVE: u8 = 0x0A;
pub const BCD_SIGN_NEGATIVE: u8 = 0x0B;
pub const F64_EXP_BIAS: i32 = 1023;
pub const F80_EXP_BIAS: i32 = 16383;
pub const MANTISSA_BITS: u32 = 64;
const F64_FRACTION_BITS: u32 = f64::MANTISSA_DIGITS - 1;
const F64_FRACTION_MASK: u64 = (1 << F64_FRACTION_BITS) - 1;
const F64_MAX_EXPONENT: u64 = (F64_EXP_BIAS as u64) * 2 + 1;
const F64_ALIGNMENT: u32 = MANTISSA_BITS - f64::MANTISSA_DIGITS;

const EXTENDED_NEAREST: u16 = 0x037f;
const GUARD_BITS: u32 = 3;
const QUIET_BIT: u128 = INT_BIT_MASK >> 1;

fn shift_with_sticky(value: u128, shift: u32) -> u128 {
    let high = value.checked_shr(shift).unwrap_or(0);
    let discarded = high.checked_shl(shift).unwrap_or(0) != value;
    high | u128::from(discarded)
}

pub const SIGN_SHIFT: u32 = 79;
pub const EXP_SHIFT: u32 = 64;
pub const INT_BIT_SHIFT: u32 = 63;

// f80 emulation
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct F80 {
    pub st: u128,
}

impl Default for F80 {
    fn default() -> Self {
        Self::new()
    }
}

impl F80 {
    pub fn new() -> Self {
        F80 { st: 0 }
    }

    pub fn PI() -> Self {
        F80 { st: F80_PI }
    }

    pub fn QNaN() -> Self {
        F80 { st: QNAN }
    }

    pub fn set_PI(&mut self) {
        self.st = F80_PI;
    }

    pub fn set_QNaN(&mut self) {
        self.st = QNAN;
    }

    pub fn set_real_indefinite(&mut self) {
        self.st = REAL_INDEFINITE;
    }

    pub fn is_invalid(&self) -> bool {
        let exp = self.get_exponent();
        let frac = self.get_fraction();

        exp == 0x7FFF && frac != 0
    }

    pub fn is_empty(&self) -> bool {
        self.is_zero() && self.is_invalid()
    }

    pub fn set_bytes(&mut self, bytes: &[u8; 10]) {
        let mut val: u128 = 0;
        for (i, &b) in bytes.iter().enumerate() {
            val |= (b as u128) << (8 * i);
        }
        self.st = val & FPU_80_BITS_MAX;
    }

    pub fn get_bytes(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = ((self.st >> (8 * i)) & 0xFF) as u8;
        }
        bytes
    }

    pub fn get_round_f64(&self, decimals: u32) -> f64 {
        let value = self.get_f64();
        let factor = 10f64.powi(decimals as i32);
        (value * factor).round() / factor
    }

    pub fn get(&self) -> u128 {
        self.st & FPU_80_BITS_MAX
    }

    pub fn set(&mut self, value: u128) {
        self.st = value & FPU_80_BITS_MAX;
    }

    pub fn fix(&mut self) {
        self.st &= FPU_80_BITS_MAX;
    }

    pub fn is_negative(&self) -> bool {
        self.st & SIGN_MASK != 0
    }

    pub fn get_sign(&self) -> bool {
        (self.st >> SIGN_SHIFT) & 1 != 0
    }

    pub fn neg(&mut self) {
        self.st ^= SIGN_MASK;
    }

    pub fn set_sign(&mut self, sign: bool) {
        self.st = (self.st & !SIGN_MASK) | ((sign as u128) << 79);
    }

    pub fn is_zero(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        let mantissa = self.st & ((1u128 << 64) - 1);
        exponent == 0 && mantissa == 0
    }

    pub fn get_integer_bit(&self) -> u64 {
        ((self.st >> INT_BIT_SHIFT) & 1) as u64
    }

    pub fn get_fraction(&self) -> u64 {
        (self.st & ((1u128 << 63) - 1)) as u64
    }

    pub fn get_exponent(&self) -> u16 {
        ((self.st >> EXP_SHIFT) & 0x7FFF) as u16
        //((self.st >> 64) & EXP_MASK) as u16
    }

    pub fn set_exponent(&mut self, exponent: u16) {
        self.st = (self.st & !EXP_MASK2) | ((exponent as u128) << 64);
    }

    pub fn get_mantissa(&self) -> u64 {
        (self.get_integer_bit() << 63) | self.get_fraction()
        /*
        let mantissa = (self.st & MANTISSA_MASK_NOINT) >> 11;
        let bit_integer = (self.st & INT_BIT_MASK) >> 63;
        ((bit_integer << 52) | mantissa) as u64*/
    }

    pub fn set_mantissa(&mut self, mantissa: u64) {
        let int_bit = (mantissa >> 63) as u128;
        let fraction = (mantissa & 0x7FFF_FFFF_FFFF_FFFF) as u128;
        self.st &= !((1u128 << INT_BIT_SHIFT) | ((1u128 << 63) - 1));
        self.st |= (int_bit << INT_BIT_SHIFT) | fraction;

        /*
        let bit_integer = ((mantissa >> 52) & 1) as u128;
        let fraction = (mantissa & 0x000F_FFFF_FFFF_FFFF) as u128; // solo los 52 bits bajos
        self.st &= !MANTISSA_MASK;
        self.st |= (bit_integer << 63) | (fraction << 11);*/
    }

    fn get_mantissa_with_integer_bit(&self) -> u64 {
        let mantissa = self.st & MANTISSA_MASK_NOINT;
        let int_bit = ((self.st & INT_BIT_MASK) >> 63) as u64;
        (mantissa as u64) | (int_bit << 63)
    }

    fn propagate_nan(&mut self, other: F80) -> bool {
        let unsupported = |x: F80| x.get_exponent() != 0 && !x.bit_integer();
        if unsupported(*self) || unsupported(other) {
            self.set_real_indefinite();
            return true;
        }
        if self.is_nan() || other.is_nan() {
            if !self.is_nan() || (other.is_nan() && other.get_mantissa() > self.get_mantissa()) {
                *self = other;
            }
            self.st |= QUIET_BIT;
            return true;
        }
        false
    }

    pub fn add(&mut self, other: F80) {
        self.add_with_control(other, false, EXTENDED_NEAREST);
    }

    pub fn sub(&mut self, other: F80) {
        self.add_with_control(other, true, EXTENDED_NEAREST);
    }

    pub(super) fn add_with_control(&mut self, mut other: F80, subtract: bool, control: u16) {
        if self.propagate_nan(other) {
            return;
        }
        if subtract {
            other.neg();
        }
        if self.is_infinite() || other.is_infinite() {
            if self.is_infinite() && other.is_infinite() && self.get_sign() != other.get_sign() {
                self.set_real_indefinite();
            } else if other.is_infinite() {
                *self = other;
            }
            return;
        }

        // Subnormals and the smallest normals have the same exponent scale.
        let parts = |x: F80| (x.get_exponent().max(1), x.get_mantissa());
        let (larger, smaller) = if parts(*self) >= parts(other) {
            (*self, other)
        } else {
            (other, *self)
        };
        let (mut exponent, larger_mantissa) = parts(larger);
        let (smaller_exponent, smaller_mantissa) = parts(smaller);
        let a = u128::from(larger_mantissa) << GUARD_BITS;
        let b = shift_with_sticky(
            u128::from(smaller_mantissa) << GUARD_BITS,
            u32::from(exponent - smaller_exponent),
        );
        let same_sign = larger.get_sign() == smaller.get_sign();
        let mut sum = if same_sign { a + b } else { a - b };
        let rounding = (control >> 10) & 3;
        let negative = larger.get_sign();
        if sum == 0 {
            let negative_zero = if same_sign { negative } else { rounding == 1 };
            self.st = if negative_zero { SIGN_MASK } else { 0 };
            return;
        }

        let leading_bit = INT_BIT_SHIFT + GUARD_BITS;
        if sum.ilog2() > leading_bit {
            sum = shift_with_sticky(sum, 1);
            exponent += 1;
        } else {
            let left = (leading_bit - sum.ilog2()).min(u32::from(exponent - 1));
            sum <<= left;
            exponent -= left as u16;
        }

        self.set_rounded(sum, i32::from(exponent), negative, control);
    }

    fn set_rounded(&mut self, mut sum: u128, mut exponent: i32, negative: bool, control: u16) {
        if exponent < 1 {
            sum = shift_with_sticky(sum, (1 - exponent) as u32);
            exponent = 1;
        }
        let rounding = (control >> 10) & 3;
        // Precision control selects 24, 53 or 64 significand bits; the exponent range stays extended.
        let precision = match (control >> 8) & 3 {
            0 => f32::MANTISSA_DIGITS,
            2 => f64::MANTISSA_DIGITS,
            _ => MANTISSA_BITS,
        };
        let discard = MANTISSA_BITS - precision + GUARD_BITS;
        let half = 1 << (discard - 1);
        let mut mantissa = sum >> discard;
        let tail = sum & ((half << 1) - 1);
        let away = match rounding {
            0 => tail > half || (tail == half && mantissa & 1 != 0),
            1 => negative && tail != 0,
            2 => !negative && tail != 0,
            _ => false,
        };
        mantissa = (mantissa + u128::from(away)) << (MANTISSA_BITS - precision);
        if mantissa > MANTISSA_MASK {
            mantissa >>= 1;
            exponent += 1;
        }
        if exponent >= EXP_MASK as i32 {
            if rounding == 0 || (rounding == 1 && negative) || (rounding == 2 && !negative) {
                exponent = EXP_MASK as i32;
                mantissa = INT_BIT_MASK;
            } else {
                exponent = EXP_MASK as i32 - 1;
                mantissa =
                    (MANTISSA_MASK >> (MANTISSA_BITS - precision)) << (MANTISSA_BITS - precision);
            }
        } else if mantissa < INT_BIT_MASK {
            exponent = 0;
        }
        self.st = ((exponent as u128) << EXP_SHIFT) | mantissa;
        self.set_sign(negative);
    }

    fn normalized_parts(self) -> (i32, u128) {
        let significand = self.get_mantissa();
        let shift = significand.leading_zeros();
        (
            i32::from(self.get_exponent().max(1)) - shift as i32,
            u128::from(significand << shift),
        )
    }

    pub(crate) fn divide(&mut self, divisor: F80, control: u16) {
        if self.propagate_nan(divisor) {
            return;
        }
        let negative = self.get_sign() != divisor.get_sign();
        let sign = if negative { SIGN_MASK } else { 0 };
        if (self.is_zero() && divisor.is_zero()) || (self.is_infinite() && divisor.is_infinite()) {
            self.set_real_indefinite();
        } else if self.is_infinite() || divisor.is_zero() {
            self.st = sign | (EXP_MASK << EXP_SHIFT) | INT_BIT_MASK;
        } else if self.is_zero() || divisor.is_infinite() {
            self.st = sign;
        } else {
            let (a_exp, a) = self.normalized_parts();
            let (b_exp, b) = divisor.normalized_parts();
            let below_one = a < b;
            let exponent = a_exp - b_exp + F80_EXP_BIAS - i32::from(below_one);
            let numerator = a << (INT_BIT_SHIFT + u32::from(below_one));
            let mantissa = numerator / b;
            let tail = (numerator % b) << GUARD_BITS;
            let sum = (mantissa << GUARD_BITS) | (tail / b) | u128::from(!tail.is_multiple_of(b));
            self.set_rounded(sum, exponent, negative, control);
        }
    }

    // A complete FPREM reports the low three quotient bits. For a partial
    // reduction Intel permits eliminating 32 to 63 exponent bits per step.
    pub(crate) fn remainder(&mut self, divisor: F80) -> Option<u8> {
        if self.get_exponent() == 0 && self.bit_integer() {
            self.set_exponent(1);
        }
        if self.propagate_nan(divisor) {
            return Some(0);
        }
        if self.is_infinite() || divisor.is_zero() {
            self.set_real_indefinite();
            return Some(0);
        }
        if self.is_zero() || divisor.is_infinite() {
            return Some(0);
        }
        let (a_exp, a) = self.normalized_parts();
        let (b_exp, b) = divisor.normalized_parts();
        let gap = a_exp - b_exp;
        if gap < 0 {
            return Some(0);
        }
        let partial = gap >= MANTISSA_BITS as i32;
        let shift = if partial { 32 } else { gap };
        let scaled = a << shift;
        let remainder = (scaled % b) as u64;
        let negative = self.get_sign();
        if remainder == 0 {
            self.st = if negative { SIGN_MASK } else { 0 };
        } else {
            let normalize = remainder.leading_zeros();
            let significand = u128::from(remainder << normalize) << GUARD_BITS;
            self.set_rounded(
                significand,
                a_exp - shift - normalize as i32,
                negative,
                EXTENDED_NEAREST,
            );
        }
        (!partial).then(|| ((scaled / b) & 7) as u8)
    }

    pub(crate) fn numeric_cmp(self, other: F80) -> Option<Ordering> {
        if [self, other]
            .iter()
            .any(|x| x.is_nan() || (x.get_exponent() != 0 && !x.bit_integer()))
        {
            return None;
        }
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        if self.get_sign() != other.get_sign() {
            return Some(other.get_sign().cmp(&self.get_sign()));
        }
        let magnitude = |x: F80| (x.get_exponent().max(1), x.get_mantissa());
        let order = magnitude(self).cmp(&magnitude(other));
        Some(if self.get_sign() {
            order.reverse()
        } else {
            order
        })
    }

    pub(crate) fn signed_integer(self, width: u32, control: u16) -> Option<i64> {
        if self.get_exponent() == EXP_MASK as u16
            || (self.get_exponent() != 0 && !self.bit_integer())
        {
            return None;
        }
        let exponent = i32::from(self.get_exponent().max(1)) - F80_EXP_BIAS;
        if exponent > INT_BIT_SHIFT as i32 {
            return None;
        }
        let shift = (INT_BIT_SHIFT as i32 - exponent) as u32;
        let significand = self.get_mantissa();
        let whole = significand.checked_shr(shift).unwrap_or(0);
        let tail = significand - whole.checked_shl(shift).unwrap_or(0);
        let rounding = (control >> 10) & 3;
        let negative = self.get_sign();
        let away = match rounding {
            0 if (1..=MANTISSA_BITS).contains(&shift) => {
                let half = 1 << (shift - 1);
                tail > half || (tail == half && whole & 1 != 0)
            }
            1 => negative && tail != 0,
            2 => !negative && tail != 0,
            _ => false,
        };
        let magnitude = i128::from(whole) + i128::from(away);
        let limit = 1i128 << (width - 1);
        if (negative && magnitude > limit) || (!negative && magnitude >= limit) {
            return None;
        }
        Some(if negative { -magnitude } else { magnitude } as i64)
    }

    pub fn is_integer(&self) -> bool {
        self.get_mantissa() == 0
    }

    pub fn is_normal(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent != 0 && exponent != 0x7FFF
    }

    pub fn bit_integer(&self) -> bool {
        self.st & INT_BIT_MASK != 0
    }

    pub fn is_denormal(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent == 0
    }

    pub fn is_infinite(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent == EXP_MASK && self.get_fraction() == 0
    }

    pub fn is_nan(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent == EXP_MASK && self.get_fraction() != 0
    }

    pub fn to_bcd_packed(&self) -> [u8; 10] {
        let mut val = self.to_integer_u128();
        let mut bcd = [0u8; 10];

        for item in bcd.iter_mut().take(9) {
            let lo = (val % 10) as u8;
            val /= 10;
            let hi = (val % 10) as u8;
            val /= 10;

            *item = (hi << 4) | lo;
        }

        bcd[9] = if self.is_negative() {
            BCD_SIGN_NEGATIVE
        } else {
            BCD_SIGN_POSITIVE
        };

        bcd
    }

    pub fn from_bcd_packed(&mut self, bcd: &[u8; 10]) {
        let mut value: u128 = 0;

        for i in (0..9).rev() {
            let byte = bcd[i];
            let hi = (byte >> 4) & 0x0F;
            let lo = byte & 0x0F;

            assert!(hi <= 9 && lo <= 9, "Invalid BCD digit");

            value = value * 100 + (hi as u128) * 10 + (lo as u128);
        }

        let is_negative = match bcd[9] & 0x0F {
            BCD_SIGN_NEGATIVE => true,
            BCD_SIGN_POSITIVE => false,
            _ => {
                log::warn!("f80: invalid BCD sign byte 0x{:x}", bcd[9]);
                false
            }
        };

        self.st = F80::encode_from_u128(value, is_negative);
    }

    pub fn to_integer_u128(&self) -> u128 {
        let exp = self.get_exponent();
        if exp == 0 || exp == 0x7FFF {
            return 0; // NaN, infinite, or cero
        }

        let bias = 16383;
        let actual_exp = exp as i32 - bias;
        if actual_exp < 0 {
            return 0; // less than 1
        }

        let mantissa = self.get_mantissa_with_integer_bit() as u128;
        if actual_exp > 63 {
            mantissa << (actual_exp as u32 - 63)
        } else {
            mantissa >> (63 - actual_exp) as u32
        }
    }

    pub fn encode_from_u128(value: u128, is_negative: bool) -> u128 {
        if value == 0 {
            return if is_negative { 1u128 << 79 } else { 0 };
        }

        let msb = 127 - value.leading_zeros() as u16;
        let exponent = 16383 + msb;
        let shift = msb as i32 - 63;

        let mantissa = if shift > 0 {
            value >> shift
        } else {
            value << (-shift) as u32
        };

        let sign_bit = if is_negative { 1u128 << 79 } else { 0 };
        let exp_bits = (exponent as u128) << 64;
        let mantissa_bits = mantissa & 0xFFFFFFFFFFFFFFFF;

        sign_bit | exp_bits | mantissa_bits
    }

    pub fn set_f64(&mut self, value: f64) {
        let bits = value.to_bits();
        let sign = if value.is_sign_negative() {
            SIGN_MASK
        } else {
            0
        };
        let exponent = ((bits >> F64_FRACTION_BITS) & F64_MAX_EXPONENT) as i32;
        let fraction = bits & F64_FRACTION_MASK;
        let (exponent, significand) = match (exponent, fraction) {
            (0, 0) => (0, 0),
            (0, fraction) => {
                let shift = fraction.leading_zeros();
                let exponent =
                    F80_EXP_BIAS + 1 - F64_EXP_BIAS + F64_ALIGNMENT as i32 - shift as i32;
                (exponent as u128, fraction << shift)
            }
            (exp, _) if exp == F64_MAX_EXPONENT as i32 => {
                (EXP_MASK, INT_BIT_MASK64 | (fraction << F64_ALIGNMENT))
            }
            _ => (
                (exponent - F64_EXP_BIAS + F80_EXP_BIAS) as u128,
                INT_BIT_MASK64 | (fraction << F64_ALIGNMENT),
            ),
        };
        self.set(sign | (exponent << EXP_SHIFT) | u128::from(significand));
    }

    pub fn get_f64(&self) -> f64 {
        let sign = if self.get_sign() { 1u64 << 63 } else { 0 };
        let exponent = i32::from(self.get_exponent());
        let significand = self.get() & MANTISSA_MASK;
        let magnitude = if exponent == 0 {
            0
        } else if exponent == EXP_MASK as i32 {
            let fraction = self.get_fraction();
            let mut payload = fraction >> F64_ALIGNMENT;
            if payload == 0 && fraction != 0 {
                payload = 1;
            }
            (F64_MAX_EXPONENT << F64_FRACTION_BITS) | payload
        } else {
            let biased = exponent - F80_EXP_BIAS + F64_EXP_BIAS;
            if biased >= F64_MAX_EXPONENT as i32 {
                F64_MAX_EXPONENT << F64_FRACTION_BITS
            } else {
                let shift = F64_ALIGNMENT as i32 + (1 - biased).max(0);
                if shift > MANTISSA_BITS as i32 {
                    0
                } else {
                    let high = significand >> shift;
                    let half = 1u128 << (shift - 1);
                    let low = significand & ((half << 1) - 1);
                    let rounded = high + u128::from(low > half || (low == half && high & 1 != 0));
                    (((biased - 1).max(0) as u64) << F64_FRACTION_BITS) + rounded as u64
                }
            }
        };
        f64::from_bits(sign | magnitude)
    }
}
