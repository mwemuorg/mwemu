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
        for i in 0..10 {
            bytes[i] = ((self.st >> (8 * i)) & 0xFF) as u8;
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
        let mantissa = self.st & MANTISSA_MASK_NOINT ;
        let int_bit = ((self.st & INT_BIT_MASK) >> 63) as u64;
        (mantissa as u64) | (int_bit << 63)
    }

    fn normalize_add(&self, mantissa: u64, exponent: u16) -> (u64, u16) {
        let mut man = mantissa;
        let mut exp = exponent;
        if man == 0 {
            return (0, 0);
        }
        while (man & INT_BIT_MASK64) == 0 {
            man <<= 1;
            exp -= 1;
        }
        (man, exp)
    }

    pub fn add(&mut self, b: F80) {
        let a_sign = self.get_sign();
        let b_sign = b.get_sign();

        let mut a_exp = self.get_exponent();
        let b_exp = b.get_exponent();

        let mut a_man = self.get_mantissa() as u128;
        let mut b_man = b.get_mantissa() as u128;

        if a_exp > b_exp {
            let shift = (a_exp - b_exp) as u32;
            b_man >>= shift;
        } else if b_exp > a_exp {
            let shift = (b_exp - a_exp) as u32;
            a_man >>= shift;
            a_exp = b_exp;
        }

        let mut result_sign;
        let mut result_exp = a_exp;
        let mut result_man: u128;

        if a_sign == b_sign {
            result_man = a_man + b_man;
            if (result_man & (1u128 << 64)) != 0 {
                result_man >>= 1;
                result_exp += 1;
            }
            result_sign = a_sign;
        } else {
            if a_man >= b_man {
                result_man = a_man - b_man;
                result_sign = a_sign;
            } else {
                result_man = b_man - a_man;
                result_sign = b_sign;
            }
            let (norm_man, norm_exp) = self.normalize_add(result_man as u64, result_exp);
            result_man = norm_man as u128;
            result_exp = norm_exp;

            if result_man == 0 {
                result_sign = false;
                result_exp = 0;
            }
        }

        self.set_sign(result_sign);
        self.set_exponent(result_exp);
        self.set_mantissa(result_man as u64);
    }

    pub fn sub(&mut self, b: F80) {
        let a_sign = self.get_sign();
        let b_sign = b.get_sign();

        let mut a_exp = self.get_exponent();
        let b_exp = b.get_exponent();

        let mut a_man = self.get_mantissa() as u128;
        let mut b_man = b.get_mantissa() as u128;

        if a_exp > b_exp {
            let shift = (a_exp - b_exp) as u32;
            b_man >>= shift;
        } else if b_exp > a_exp {
            let shift = (b_exp - a_exp) as u32;
            a_man >>= shift;
            a_exp = b_exp;
        }

        let mut result_sign;
        let mut result_exp = a_exp;
        let mut result_man: u128;

        if a_sign != b_sign {
            result_man = a_man + b_man;
            if (result_man & (1u128 << 64)) != 0 {
                result_man >>= 1;
                result_exp += 1;
            }
            result_sign = a_sign;
        } else {
            if a_man >= b_man {
                result_man = a_man - b_man;
                result_sign = a_sign;
            } else {
                result_man = b_man - a_man;
                result_sign = !a_sign;
            }
            let (norm_man, norm_exp) = self.normalize_sub(result_man as u64, result_exp);
            result_man = norm_man as u128;
            result_exp = norm_exp;

            if result_man == 0 {
                result_sign = false; // +0
                result_exp = 0;
            }
        }

        self.set_sign(result_sign);
        self.set_exponent(result_exp);
        self.set_mantissa(result_man as u64);
    }

    fn normalize_sub(&self, mut mant: u64, mut exp: u16) -> (u64, u16) {
        while mant != 0 && (mant & (1 << 63)) == 0 {
            mant <<= 1;
            exp -= 1;
        }
        (mant, exp)
    }

    pub fn is_integer(&self) -> bool {
        self.get_mantissa() == 0
    }

    pub fn is_normal(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent != 0 && exponent != 0x7FFF
    }

    pub fn bit_integer(&self) -> bool {
        (self.st & INT_BIT_MASK) & 1 == 1
    }

    pub fn is_denormal(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent == 0
    }

    pub fn is_infinite(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        let mantissa = self.st & ((1u128 << 64) - 1);
        exponent == 0x7FFF && mantissa == 0
    }

    pub fn is_nan(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        let mantissa = self.st & MANTISSA_MASK;
        exponent == 0x7FFF && mantissa != 0
    }

    pub fn to_bcd_packed(&self) -> [u8; 10] {
        let mut val = self.to_integer_u128();
        let mut bcd = [0u8; 10];

        for i in 0..9 {
            let lo = (val % 10) as u8;
            val /= 10;
            let hi = (val % 10) as u8;
            val /= 10;

            bcd[i] = (hi << 4) | lo;
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
            _ => panic!("Invalid BCD sign"),
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
        let sign = (bits >> 63) & 1;
        let exp = ((bits >> 52) & 0x7FF) as u16;
        let mantissa = bits & 0xFFFFFFFFFFFFF;

        let st = if exp == 0 {
            // Subnormal or zero in f64 → represent as 0.0 en FPU
            (sign as u128) << 79
        } else if exp == 0x7FF {
            // Inf o NaN
            let is_nan = mantissa != 0;
            let extended_exp = 0x7FFFu128;
            let extended_mantissa = (mantissa as u128) << 11;
            let nan_bit = if is_nan { 1u128 << 62 } else { 0 }; // QNaN bit (optional)
            ((sign as u128) << 79) | (extended_exp << 64) | extended_mantissa | nan_bit
        } else {
            // Normal number
            let adjusted_exp = (exp as i32 - F64_EXP_BIAS) + F80_EXP_BIAS;
            let extended_exp = adjusted_exp as u128;
            let full_mantissa = ((1u64 << 52) | mantissa) as u128; // add implicit bit
            let extended_mantissa = full_mantissa << 11; // 63 bits align
            ((sign as u128) << 79) | (extended_exp << 64) | extended_mantissa
        };

        self.set(st); // masked setter
    }

    pub fn get_f64(&self) -> f64 {
        let value = self.get();
        let sign = ((value >> 79) & 1) as u64;
        let exp = ((value >> 64) & 0x7FFF) as u16;
        let mantissa = value & 0xFFFFFFFFFFFFFFFF;

        let f64_bits: u64 = if exp == 0 {
            // Zero or denormal extended → 0.0
            sign << 63
        } else if exp == 0x7FFF {
            // Inf or NaN
            let f64_mantissa = (mantissa >> 11) as u64;
            let nan_mask = if f64_mantissa != 0 { 1 << 51 } else { 0 }; // set QNaN bit if needed
            (sign << 63) | (0x7FF << 52) | f64_mantissa | nan_mask
        } else {
            let unbiased_exp = exp as i32 - F80_EXP_BIAS;
            let f64_exp = unbiased_exp + F64_EXP_BIAS;

            if f64_exp <= 0 {
                // Subnormal in f64 → round to 0.0
                sign << 63
            } else if f64_exp >= 0x7FF {
                // Exponent too large → ∞
                (sign << 63) | (0x7FF << 52)
            } else {
                let f64_mantissa = ((mantissa >> 11) & 0xFFFFFFFFFFFFF) as u64;
                (sign << 63) | ((f64_exp as u64) << 52) | f64_mantissa
            }
        };

        f64::from_bits(f64_bits)
    }
}
