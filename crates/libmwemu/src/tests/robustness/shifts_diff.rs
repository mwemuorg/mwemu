//! Differential shift/rotate flag testing vs the real x86_64 CPU.
//!
//! Shifts are the classic flag-bug hotspot: count masking (5-bit for 8/16/32,
//! 6-bit for 64), CF = last bit out, and OF that is *only architecturally
//! defined for count==1*. We therefore compare CF/SF/ZF/PF whenever the count
//! is non-zero, OF only when count==1, and never AF (undefined for shifts).
//! Host-gated to x86_64.
#![cfg(target_arch = "x86_64")]

use crate::arch::x86::flags::Flags;
use std::arch::asm;

const VALS8: &[u8] = &[0x00, 0x01, 0x80, 0x81, 0xff, 0x7f, 0xaa, 0x55, 0x0f, 0xf0];
const COUNTS: &[u8] = &[0, 1, 2, 7, 8, 15, 16, 17, 31];

/// Compare mwemu's post-op flags against RFLAGS, honoring which flags a shift
/// actually defines for this `count`.
fn check(op: &str, val: u64, count: u8, rf: u64, f: &mut Flags) {
    f.materialize_lazy();
    if count == 0 {
        return; // shifts by 0 leave flags untouched — nothing to compare
    }
    let cpu_cf = rf & (1 << 0) != 0;
    let cpu_pf = rf & (1 << 2) != 0;
    let cpu_zf = rf & (1 << 6) != 0;
    let cpu_sf = rf & (1 << 7) != 0;
    let cpu_of = rf & (1 << 11) != 0;
    assert_eq!(f.f_cf, cpu_cf, "{op}({val:#x}, {count}) CF");
    assert_eq!(f.f_zf, cpu_zf, "{op}({val:#x}, {count}) ZF");
    assert_eq!(f.f_sf, cpu_sf, "{op}({val:#x}, {count}) SF");
    assert_eq!(f.f_pf, cpu_pf, "{op}({val:#x}, {count}) PF");
    if count == 1 {
        assert_eq!(f.f_of, cpu_of, "{op}({val:#x}, {count}) OF");
    }
}

// ---- 8-bit variable-count shifts ----

#[test]
fn shl8_matches_cpu() {
    for &v in VALS8 {
        for &c in COUNTS {
            let (res_cpu, rf): (u8, u64);
            unsafe {
                asm!("shl {a}, cl", "pushfq", "pop {rf}",
                    a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shl2p8(v as u64, c as u64) as u8;
            assert_eq!(res_cpu, res_m, "shl8({v:#x},{c}) result");
            check("shl8", v as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn shr8_matches_cpu() {
    for &v in VALS8 {
        for &c in COUNTS {
            let (res_cpu, rf): (u8, u64);
            unsafe {
                asm!("shr {a}, cl", "pushfq", "pop {rf}",
                    a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shr2p8(v as u64, c as u64) as u8;
            assert_eq!(res_cpu, res_m, "shr8({v:#x},{c}) result");
            check("shr8", v as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn sar8_matches_cpu() {
    for &v in VALS8 {
        for &c in COUNTS {
            let (res_cpu, rf): (u8, u64);
            unsafe {
                asm!("sar {a}, cl", "pushfq", "pop {rf}",
                    a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.sar2p8(v as u64, c as u64) as u8;
            assert_eq!(res_cpu, res_m, "sar8({v:#x},{c}) result");
            check("sar8", v as u64, c, rf, &mut f);
        }
    }
}

// ---- 16-bit variable-count shifts (exercises the sar count>=width sign path) ----

const VALS16: &[u16] = &[0, 1, 0x8000, 0x7fff, 0xffff, 0xaaaa, 0x00ff, 0x0080];

#[test]
fn shl16_matches_cpu() {
    for &v in VALS16 {
        for &c in COUNTS {
            let (res_cpu, rf): (u16, u64);
            unsafe {
                asm!("shl {a:x}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shl2p16(v as u64, c as u64) as u16;
            assert_eq!(res_cpu, res_m, "shl16({v:#x},{c}) result");
            check("shl16", v as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn shr16_matches_cpu() {
    for &v in VALS16 {
        for &c in COUNTS {
            let (res_cpu, rf): (u16, u64);
            unsafe {
                asm!("shr {a:x}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shr2p16(v as u64, c as u64) as u16;
            assert_eq!(res_cpu, res_m, "shr16({v:#x},{c}) result");
            check("shr16", v as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn sar16_matches_cpu() {
    for &v in VALS16 {
        for &c in COUNTS {
            let (res_cpu, rf): (u16, u64);
            unsafe {
                asm!("sar {a:x}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.sar2p16(v as u64, c as u64) as u16;
            assert_eq!(res_cpu, res_m, "sar16({v:#x},{c}) result");
            check("sar16", v as u64, c, rf, &mut f);
        }
    }
}

// ---- 32-bit variable-count shifts ----

const VALS32: &[u32] = &[0, 1, 0x8000_0000, 0x7fff_ffff, 0xffff_ffff, 0xaaaa_aaaa, 0x0000_00ff];

#[test]
fn shl32_matches_cpu() {
    for &v in VALS32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("shl {a:e}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shl2p32(v as u64, c as u64) as u32;
            assert_eq!(res_cpu, res_m, "shl32({v:#x},{c}) result");
            check("shl32", v as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn shr32_matches_cpu() {
    for &v in VALS32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("shr {a:e}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shr2p32(v as u64, c as u64) as u32;
            assert_eq!(res_cpu, res_m, "shr32({v:#x},{c}) result");
            check("shr32", v as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn sar32_matches_cpu() {
    for &v in VALS32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("sar {a:e}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.sar2p32(v as u64, c as u64) as u32;
            assert_eq!(res_cpu, res_m, "sar32({v:#x},{c}) result");
            check("sar32", v as u64, c, rf, &mut f);
        }
    }
}

// ---- shld / shrd (double-precision shifts) ----
// For 32-bit, every masked count (0..31) is architecturally defined.

const DBL32: &[(u32, u32)] = &[
    (0, 0), (1, 0), (0x8000_0000, 0xffff_ffff), (0xdead_beef, 0x1234_5678),
    (0xffff_ffff, 0), (0, 0xffff_ffff), (0x7fff_ffff, 0x8000_0000), (0xaaaa_aaaa, 0x5555_5555),
];

#[test]
fn shld32_matches_cpu() {
    for &(v0, v1) in DBL32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("shld {d:e}, {s:e}, cl", "pushfq", "pop {rf}",
                    d = inout(reg) v0 => res_cpu, s = in(reg) v1, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shld(v0 as u64, v1 as u64, c as u64, 32) as u32;
            assert_eq!(res_cpu, res_m, "shld32({v0:#x},{v1:#x},{c}) result");
            check("shld32", v0 as u64, c, rf, &mut f);
        }
    }
}

#[test]
fn shrd32_matches_cpu() {
    for &(v0, v1) in DBL32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("shrd {d:e}, {s:e}, cl", "pushfq", "pop {rf}",
                    d = inout(reg) v0 => res_cpu, s = in(reg) v1, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shrd(v0 as u64, v1 as u64, c as u64, 32) as u32;
            assert_eq!(res_cpu, res_m, "shrd32({v0:#x},{v1:#x},{c}) result");
            check("shrd32", v0 as u64, c, rf, &mut f);
        }
    }
}

const DBL64: &[(u64, u64)] = &[
    (0, 0), (1, 0), (0x8000_0000_0000_0000, u64::MAX), (0xdead_beef_cafe_babe, 0x0123_4567_89ab_cdef),
    (u64::MAX, 0), (0, u64::MAX),
];
const COUNTS64: &[u8] = &[0, 1, 2, 31, 32, 33, 63];

#[test]
fn shld64_matches_cpu() {
    for &(v0, v1) in DBL64 {
        for &c in COUNTS64 {
            let (res_cpu, rf): (u64, u64);
            unsafe {
                asm!("shld {d:r}, {s:r}, cl", "pushfq", "pop {rf}",
                    d = inout(reg) v0 => res_cpu, s = in(reg) v1, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shld(v0, v1, c as u64, 64);
            assert_eq!(res_cpu, res_m, "shld64({v0:#x},{v1:#x},{c}) result");
            check("shld64", v0, c, rf, &mut f);
        }
    }
}

#[test]
fn shrd64_matches_cpu() {
    for &(v0, v1) in DBL64 {
        for &c in COUNTS64 {
            let (res_cpu, rf): (u64, u64);
            unsafe {
                asm!("shrd {d:r}, {s:r}, cl", "pushfq", "pop {rf}",
                    d = inout(reg) v0 => res_cpu, s = in(reg) v1, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.shrd(v0, v1, c as u64, 64);
            assert_eq!(res_cpu, res_m, "shrd64({v0:#x},{v1:#x},{c}) result");
            check("shrd64", v0, c, rf, &mut f);
        }
    }
}
