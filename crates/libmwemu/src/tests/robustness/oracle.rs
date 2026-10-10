//! Pure-Rust reference model for x86 integer flag semantics, written straight
//! from the Intel SDM pseudocode. It is the oracle the `*_diff` tests compare
//! `flags.rs` against.
//!
//! It deliberately shares no code with the emulator: results come from u128 /
//! i128 arithmetic and `rotate_left`-style std helpers, flags from the SDM
//! formulas, so a bug in `flags.rs` can't be mirrored here. Only the flags an
//! instruction architecturally defines are set; the rest stay 0 and the tests
//! never compare them.
//!
//! Every function returns `(result, rflags)` with the result already masked to
//! `w` bits and RFLAGS laid out as the hardware does (CF bit 0, PF 2, AF 4,
//! ZF 6, SF 7, OF 11), so the tests read it exactly as they used to read
//! `pushfq`.

pub const CF: u64 = 1 << 0;
pub const PF: u64 = 1 << 2;
pub const AF: u64 = 1 << 4;
pub const ZF: u64 = 1 << 6;
pub const SF: u64 = 1 << 7;
pub const OF: u64 = 1 << 11;

fn mask(w: u32) -> u64 {
    u64::MAX >> (64 - w)
}

fn msb(v: u64, w: u32) -> bool {
    (v >> (w - 1)) & 1 != 0
}

/// Shift count mask: 6 bits for 64-bit operands, 5 bits otherwise.
fn count_mask(w: u32) -> u8 {
    if w == 64 { 0x3f } else { 0x1f }
}

/// ZF/SF/PF from a result (PF = even parity of the low byte).
fn result_flags(res: u64, w: u32) -> u64 {
    let mut rf = 0;
    if res == 0 {
        rf |= ZF;
    }
    if msb(res, w) {
        rf |= SF;
    }
    if (res & 0xff).count_ones().is_multiple_of(2) {
        rf |= PF;
    }
    rf
}

fn flag(set: bool, bit: u64) -> u64 {
    if set { bit } else { 0 }
}

// ---- add / sub family ----

/// ADD (`cin = false`) / ADC (`cin = carry`).
pub fn add(a: u64, b: u64, cin: bool, w: u32) -> (u64, u64) {
    let m = mask(w);
    let (a, b) = (a & m, b & m);
    let full = a as u128 + b as u128 + cin as u128;
    let res = (full as u64) & m;
    let rf = result_flags(res, w)
        | flag(full > m as u128, CF)
        | flag(msb((a ^ res) & (b ^ res), w), OF)
        | flag((a ^ b ^ res) & 0x10 != 0, AF);
    (res, rf)
}

/// SUB (`bin = false`) / SBB (`bin = borrow`).
pub fn sub(a: u64, b: u64, bin: bool, w: u32) -> (u64, u64) {
    let m = mask(w);
    let (a, b) = (a & m, b & m);
    let res = a.wrapping_sub(b).wrapping_sub(bin as u64) & m;
    let rf = result_flags(res, w)
        | flag((a as u128) < b as u128 + bin as u128, CF)
        | flag(msb((a ^ b) & (a ^ res), w), OF)
        | flag((a ^ b ^ res) & 0x10 != 0, AF);
    (res, rf)
}

/// INC: ADD by 1 with CF preserved.
pub fn inc(a: u64, cf: bool, w: u32) -> (u64, u64) {
    let (res, rf) = add(a, 1, false, w);
    (res, (rf & !CF) | flag(cf, CF))
}

/// DEC: SUB by 1 with CF preserved.
pub fn dec(a: u64, cf: bool, w: u32) -> (u64, u64) {
    let (res, rf) = sub(a, 1, false, w);
    (res, (rf & !CF) | flag(cf, CF))
}

/// NEG: `0 - a`, CF set unless the operand is zero.
pub fn neg(a: u64, w: u32) -> (u64, u64) {
    sub(0, a, false, w)
}

// ---- mul / imul (8-bit forms) ----

/// MUL r/m8: AX = AL * r/m8, CF = OF = upper half non-zero.
pub fn mul8(a: u8, b: u8) -> (u16, u64) {
    let ax = a as u16 * b as u16;
    let over = ax >> 8 != 0;
    (ax, flag(over, CF) | flag(over, OF))
}

/// IMUL r/m8: AX = AL * r/m8 signed, CF = OF = result doesn't fit in AL.
pub fn imul8(a: u8, b: u8) -> (u16, u64) {
    let ax = (a as i8 as i16).wrapping_mul(b as i8 as i16);
    let over = ax != ax as i8 as i16;
    (ax as u16, flag(over, CF) | flag(over, OF))
}

// ---- bit tests / scans ----

/// BT reg, reg: CF = bit `bit mod w` of `v`.
pub fn bt(v: u64, bit: u64, w: u32) -> u64 {
    flag((v >> (bit % w as u64)) & 1 != 0, CF)
}

/// BSF: index of the lowest set bit; ZF = source is zero (index undefined).
pub fn bsf(v: u64) -> (Option<u64>, u64) {
    if v == 0 {
        (None, ZF)
    } else {
        (Some(v.trailing_zeros() as u64), 0)
    }
}

/// BSR: index of the highest set bit; ZF = source is zero (index undefined).
pub fn bsr(v: u64, w: u32) -> (Option<u64>, u64) {
    if v & mask(w) == 0 {
        (None, ZF)
    } else {
        (Some(63 - (v & mask(w)).leading_zeros() as u64), 0)
    }
}

// ---- shifts ----
//
// Count is masked (5/6 bits); a masked count of 0 leaves everything untouched.
// CF is the last bit shifted out, which the u128 datapath yields naturally for
// counts beyond the operand width (0 for SHL/SHR, the sign for SAR), matching
// hardware. OF is only architecturally defined for count == 1.

/// SHL r/m, cl.
pub fn shl(v: u64, c: u8, w: u32) -> (u64, u64) {
    let m = (c & count_mask(w)) as u32;
    let v = v & mask(w);
    if m == 0 {
        return (v, 0);
    }
    let full = (v as u128) << m;
    let res = (full as u64) & mask(w);
    let cf = (full >> w) & 1 != 0;
    let rf = result_flags(res, w) | flag(cf, CF) | flag(m == 1 && (msb(res, w) ^ cf), OF);
    (res, rf)
}

/// SHR r/m, cl.
pub fn shr(v: u64, c: u8, w: u32) -> (u64, u64) {
    let m = (c & count_mask(w)) as u32;
    let v = v & mask(w);
    if m == 0 {
        return (v, 0);
    }
    let res = ((v as u128) >> m) as u64;
    let cf = ((v as u128) >> (m - 1)) & 1 != 0;
    let rf = result_flags(res, w) | flag(cf, CF) | flag(m == 1 && msb(v, w), OF);
    (res, rf)
}

/// SAR r/m, cl.
pub fn sar(v: u64, c: u8, w: u32) -> (u64, u64) {
    let m = (c & count_mask(w)) as u32;
    let v = v & mask(w);
    if m == 0 {
        return (v, 0);
    }
    // Sign-extend to i128 so shifting past the width keeps producing sign bits.
    let sv = ((v << (64 - w)) as i64 >> (64 - w)) as i128;
    let res = ((sv >> m) as u64) & mask(w);
    let cf = (sv >> (m - 1)) & 1 != 0;
    // OF for SAR with count 1 is always 0.
    (res, result_flags(res, w) | flag(cf, CF))
}

/// SHLD dst, src, cl (32/64-bit: every masked count is defined).
pub fn shld(d: u64, s: u64, c: u8, w: u32) -> (u64, u64) {
    let m = (c & count_mask(w)) as u32;
    let (d, s) = (d & mask(w), s & mask(w));
    if m == 0 {
        return (d, 0);
    }
    let res = ((d << m) | (s >> (w - m))) & mask(w);
    let cf = (d >> (w - m)) & 1 != 0;
    let rf = result_flags(res, w) | flag(cf, CF) | flag(m == 1 && (msb(res, w) ^ msb(d, w)), OF);
    (res, rf)
}

/// SHRD dst, src, cl (32/64-bit: every masked count is defined).
pub fn shrd(d: u64, s: u64, c: u8, w: u32) -> (u64, u64) {
    let m = (c & count_mask(w)) as u32;
    let (d, s) = (d & mask(w), s & mask(w));
    if m == 0 {
        return (d, 0);
    }
    let res = ((d >> m) | (s << (w - m))) & mask(w);
    let cf = (d >> (m - 1)) & 1 != 0;
    let rf = result_flags(res, w) | flag(cf, CF) | flag(m == 1 && (msb(res, w) ^ msb(d, w)), OF);
    (res, rf)
}

// ---- rotates ----
//
// Rotates touch only CF and OF. The masked count decides whether flags change
// at all; the rotation itself runs `masked mod w` (ROL/ROR) or `masked mod
// (w+1)` for 8/16-bit RCL/RCR, which rotate through the carry bit.

/// ROL r/m, cl: CF = new LSB; OF (count 1) = MSB(res) ^ CF.
pub fn rol(v: u64, c: u8, w: u32) -> (u64, u64) {
    let masked = (c & count_mask(w)) as u32;
    let v = v & mask(w);
    if masked == 0 {
        return (v, 0);
    }
    let t = masked % w;
    let res = ((v << t) | (v >> ((w - t) % w))) & mask(w);
    let cf = res & 1 != 0;
    (
        res,
        flag(cf, CF) | flag(masked == 1 && (msb(res, w) ^ cf), OF),
    )
}

/// ROR r/m, cl: CF = new MSB; OF (count 1) = MSB(res) ^ MSB-1(res).
pub fn ror(v: u64, c: u8, w: u32) -> (u64, u64) {
    let masked = (c & count_mask(w)) as u32;
    let v = v & mask(w);
    if masked == 0 {
        return (v, 0);
    }
    let t = masked % w;
    let res = ((v >> t) | (v << ((w - t) % w))) & mask(w);
    let cf = msb(res, w);
    let of = msb(res, w) ^ ((res >> (w - 2)) & 1 != 0);
    (res, flag(cf, CF) | flag(masked == 1 && of, OF))
}

/// Effective RCL/RCR count: the SDM reduces 8/16-bit counts modulo `w + 1`.
fn rc_count(masked: u32, w: u32) -> u32 {
    match w {
        8 => masked % 9,
        16 => masked % 17,
        _ => masked,
    }
}

/// RCL r/m, cl with carry-in `cin`: OF (count 1) = MSB(res) ^ CF, after rotating.
pub fn rcl(v: u64, c: u8, w: u32, cin: bool) -> (u64, u64) {
    let masked = (c & count_mask(w)) as u32;
    let mut res = v & mask(w);
    if masked == 0 {
        return (res, 0);
    }
    let mut cf = cin;
    for _ in 0..rc_count(masked, w) {
        let out = msb(res, w);
        res = ((res << 1) | cf as u64) & mask(w);
        cf = out;
    }
    (
        res,
        flag(cf, CF) | flag(masked == 1 && (msb(res, w) ^ cf), OF),
    )
}

/// RCR r/m, cl with carry-in `cin`: OF (count 1) = MSB(dst) ^ CF, before rotating.
pub fn rcr(v: u64, c: u8, w: u32, cin: bool) -> (u64, u64) {
    let masked = (c & count_mask(w)) as u32;
    let mut res = v & mask(w);
    if masked == 0 {
        return (res, 0);
    }
    let of = msb(res, w) ^ cin;
    let mut cf = cin;
    for _ in 0..rc_count(masked, w) {
        let out = res & 1 != 0;
        res = (res >> 1) | ((cf as u64) << (w - 1));
        cf = out;
    }
    (res, flag(cf, CF) | flag(masked == 1 && of, OF))
}
