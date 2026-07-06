use serde::{Deserialize, Serialize};

/// AArch64 condition flags (NZCV) stored in PSTATE.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct FlagsNZCV {
    pub n: bool, // Negative
    pub z: bool, // Zero
    pub c: bool, // Carry
    pub v: bool, // oVerflow
}

impl FlagsNZCV {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Pack into the NZCV field of PSTATE (bits [31:28]).
    pub fn as_u64(&self) -> u64 {
        ((self.n as u64) << 31)
            | ((self.z as u64) << 30)
            | ((self.c as u64) << 29)
            | ((self.v as u64) << 28)
    }

    /// Unpack from a PSTATE-style NZCV value.
    pub fn from_u64(&mut self, val: u64) {
        self.n = (val >> 31) & 1 != 0;
        self.z = (val >> 30) & 1 != 0;
        self.c = (val >> 29) & 1 != 0;
        self.v = (val >> 28) & 1 != 0;
    }

    /// Update flags for the result of an addition (a + b = result).
    pub fn update_add64(&mut self, a: u64, b: u64, result: u64) {
        self.n = (result as i64) < 0;
        self.z = result == 0;
        self.c = result < a; // unsigned overflow
        self.v = ((a as i64 ^ result as i64) & (b as i64 ^ result as i64)) < 0;
    }

    pub fn update_add32(&mut self, a: u32, b: u32, result: u32) {
        self.n = (result as i32) < 0;
        self.z = result == 0;
        self.c = result < a;
        self.v = ((a as i32 ^ result as i32) & (b as i32 ^ result as i32)) < 0;
    }

    /// Update flags for the result of a subtraction (a - b = result).
    pub fn update_sub64(&mut self, a: u64, b: u64, result: u64) {
        self.n = (result as i64) < 0;
        self.z = result == 0;
        self.c = a >= b; // ARM: carry = NOT borrow
        self.v = ((a as i64 ^ b as i64) & (a as i64 ^ result as i64)) < 0;
    }

    pub fn update_sub32(&mut self, a: u32, b: u32, result: u32) {
        self.n = (result as i32) < 0;
        self.z = result == 0;
        self.c = a >= b;
        self.v = ((a as i32 ^ b as i32) & (a as i32 ^ result as i32)) < 0;
    }

    /// Update flags for a logical result (AND, ORR, EOR, etc.).
    /// Logical ops clear C and V.
    pub fn update_logic64(&mut self, result: u64) {
        self.n = (result as i64) < 0;
        self.z = result == 0;
        self.c = false;
        self.v = false;
    }

    pub fn update_logic32(&mut self, result: u32) {
        self.n = (result as i32) < 0;
        self.z = result == 0;
        self.c = false;
        self.v = false;
    }

    /// Evaluate an ARM condition code against current flags.
    pub fn eval_condition(&self, cond: u8) -> bool {
        match cond & 0xf {
            0b0000 => self.z,                        // EQ
            0b0001 => !self.z,                       // NE
            0b0010 => self.c,                        // CS/HS
            0b0011 => !self.c,                       // CC/LO
            0b0100 => self.n,                        // MI
            0b0101 => !self.n,                       // PL
            0b0110 => self.v,                        // VS
            0b0111 => !self.v,                       // VC
            0b1000 => self.c && !self.z,             // HI
            0b1001 => !self.c || self.z,             // LS
            0b1010 => self.n == self.v,              // GE
            0b1011 => self.n != self.v,              // LT
            0b1100 => !self.z && (self.n == self.v), // GT
            0b1101 => self.z || (self.n != self.v),  // LE
            0b1110 => true,                          // AL
            0b1111 => true,                          // AL (NV treated as AL)
            _ => unreachable!(),
        }
    }

    pub fn diff(a: &FlagsNZCV, b: &FlagsNZCV) -> String {
        let mut s = String::new();
        if a.n != b.n {
            s.push_str(&format!("N: {} -> {}  ", a.n, b.n));
        }
        if a.z != b.z {
            s.push_str(&format!("Z: {} -> {}  ", a.z, b.z));
        }
        if a.c != b.c {
            s.push_str(&format!("C: {} -> {}  ", a.c, b.c));
        }
        if a.v != b.v {
            s.push_str(&format!("V: {} -> {}  ", a.v, b.v));
        }
        s
    }
}

/// AArch64 register file.
///
/// X0–X30 are the 64-bit GPRs. W0–W30 are the lower 32-bit views (handled
/// via accessor methods). X31 reads as zero when used as the zero register (XZR/WZR),
/// but SP is a separate register.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RegsAarch64 {
    // General-purpose registers X0–X30
    pub x: [u64; 31],
    // Stack pointer (separate from X31)
    pub sp: u64,
    // Program counter
    pub pc: u64,
    // Condition flags
    pub nzcv: FlagsNZCV,

    // System registers (minimal set)
    pub tpidr_el0: u64, // Thread pointer (analogous to FS/GS base)
    pub fpcr: u64,      // Floating-point control register
    pub fpsr: u64,      // Floating-point status register

    // SIMD/FP registers V0–V31 (128-bit NEON)
    pub v: [u128; 32],
}

impl Default for RegsAarch64 {
    fn default() -> Self {
        Self::new()
    }
}

impl RegsAarch64 {
    pub fn new() -> Self {
        Self {
            x: [0u64; 31],
            sp: 0,
            pc: 0,
            nzcv: FlagsNZCV::new(),
            tpidr_el0: 0,
            fpcr: 0,
            fpsr: 0,
            v: [0u128; 32],
        }
    }

    /// Read a 64-bit GPR. Register 31 returns zero (XZR).
    pub fn get_x(&self, reg: usize) -> u64 {
        if reg == 31 { 0 } else { self.x[reg] }
    }

    /// Read the lower 32-bit view of a GPR. Register 31 returns zero (WZR).
    pub fn get_w(&self, reg: usize) -> u32 {
        self.get_x(reg) as u32
    }

    /// Write a 64-bit GPR. Writes to register 31 are discarded (XZR).
    pub fn set_x(&mut self, reg: usize, val: u64) {
        if reg < 31 {
            self.x[reg] = val;
        }
    }

    /// Write the lower 32-bit view, zero-extending to 64 bits.
    /// Writes to register 31 are discarded (WZR).
    pub fn set_w(&mut self, reg: usize, val: u32) {
        self.set_x(reg, val as u64);
    }

    /// Read SP or a GPR depending on whether the encoding means SP or XZR.
    /// In many load/store instructions, reg 31 encodes SP, not XZR.
    pub fn get_x_or_sp(&self, reg: usize) -> u64 {
        if reg == 31 { self.sp } else { self.x[reg] }
    }

    /// Write SP or a GPR.
    pub fn set_x_or_sp(&mut self, reg: usize, val: u64) {
        if reg == 31 {
            self.sp = val;
        } else {
            self.x[reg] = val;
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<u64> {
        let lower = name.to_lowercase();
        if lower == "sp" {
            return Some(self.sp);
        }
        if lower == "pc" {
            return Some(self.pc);
        }
        if lower == "nzcv" {
            return Some(self.nzcv.as_u64());
        }
        if lower == "tpidr_el0" {
            return Some(self.tpidr_el0);
        }
        if lower == "lr" {
            return Some(self.x[30]);
        }
        if lower == "fp" {
            return Some(self.x[29]);
        }
        if let Some(n) = lower.strip_prefix('x') {
            if let Ok(i) = n.parse::<usize>() {
                if i < 31 {
                    return Some(self.x[i]);
                }
            }
        }
        if let Some(n) = lower.strip_prefix('w') {
            if let Ok(i) = n.parse::<usize>() {
                if i < 31 {
                    return Some(self.x[i] & 0xffffffff);
                }
            }
        }
        None
    }

    pub fn set_by_name(&mut self, name: &str, val: u64) -> bool {
        let lower = name.to_lowercase();
        if lower == "sp" {
            self.sp = val;
            return true;
        }
        if lower == "pc" {
            self.pc = val;
            return true;
        }
        if lower == "lr" {
            self.x[30] = val;
            return true;
        }
        if lower == "fp" {
            self.x[29] = val;
            return true;
        }
        if let Some(n) = lower.strip_prefix('x') {
            if let Ok(i) = n.parse::<usize>() {
                if i < 31 {
                    self.x[i] = val;
                    return true;
                }
            }
        }
        if let Some(n) = lower.strip_prefix('w') {
            if let Ok(i) = n.parse::<usize>() {
                if i < 31 {
                    self.x[i] = val & 0xffffffff;
                    return true;
                }
            }
        }
        false
    }

    pub fn diff(a: &RegsAarch64, b: &RegsAarch64) -> String {
        let mut s = String::new();
        for i in 0..31 {
            if a.x[i] != b.x[i] {
                s.push_str(&format!("x{}: 0x{:x} -> 0x{:x}  ", i, a.x[i], b.x[i]));
            }
        }
        if a.sp != b.sp {
            s.push_str(&format!("sp: 0x{:x} -> 0x{:x}  ", a.sp, b.sp));
        }
        if a.pc != b.pc {
            s.push_str(&format!("pc: 0x{:x} -> 0x{:x}  ", a.pc, b.pc));
        }
        let flags_diff = FlagsNZCV::diff(&a.nzcv, &b.nzcv);
        if !flags_diff.is_empty() {
            s.push_str(&flags_diff);
        }
        s
    }
}
