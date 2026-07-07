//! Differential rotate flag testing vs the real x86_64 CPU.
//!
//! Unlike shifts, rotates (ROL/ROR/RCL/RCR) affect **only CF and OF** — SF/ZF/PF/
//! AF are left untouched — so we compare CF (whenever the masked count is
//! non-zero) and OF (only for count==1), and nothing else. RCL/RCR rotate through
//! CF, so we seed the carry-in on both sides. Host-gated to x86_64.
#![cfg(target_arch = "x86_64")]

use crate::arch::x86::flags::Flags;
use std::arch::asm;

const VALS8: &[u8] = &[0x00, 0x01, 0x80, 0x81, 0xff, 0x7f, 0xaa, 0x55, 0x0f];
const COUNTS: &[u8] = &[0, 1, 2, 7, 8, 9, 15, 16, 17, 31];

fn check_rot(op: &str, val: u64, count: u8, mask: u64, rf: u64, f: &mut Flags) {
    f.materialize_lazy();
    if (count as u64 & mask) == 0 {
        return; // masked count 0 → rotate leaves flags untouched
    }
    let cpu_cf = rf & (1 << 0) != 0;
    let cpu_of = rf & (1 << 11) != 0;
    assert_eq!(f.f_cf, cpu_cf, "{op}({val:#x}, {count}) CF");
    if count == 1 {
        assert_eq!(f.f_of, cpu_of, "{op}({val:#x}, {count}) OF");
    }
}

// ---- ROL / ROR (no carry) ----

#[test]
fn rol8_matches_cpu() {
    for &v in VALS8 {
        for &c in COUNTS {
            let (res_cpu, rf): (u8, u64);
            unsafe {
                asm!("rol {a}, cl", "pushfq", "pop {rf}",
                    a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.rol(v as u64, c as u64, 8) as u8;
            assert_eq!(res_cpu, res_m, "rol8({v:#x},{c}) result");
            check_rot("rol8", v as u64, c, 0x1f, rf, &mut f);
        }
    }
}

#[test]
fn ror8_matches_cpu() {
    for &v in VALS8 {
        for &c in COUNTS {
            let (res_cpu, rf): (u8, u64);
            unsafe {
                asm!("ror {a}, cl", "pushfq", "pop {rf}",
                    a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.ror(v as u64, c as u64, 8) as u8;
            assert_eq!(res_cpu, res_m, "ror8({v:#x},{c}) result");
            check_rot("ror8", v as u64, c, 0x1f, rf, &mut f);
        }
    }
}

const VALS16: &[u16] = &[0, 1, 0x8000, 0x7fff, 0xffff, 0xaaaa, 0x00ff, 0x0080];

#[test]
fn rol16_matches_cpu() {
    for &v in VALS16 {
        for &c in COUNTS {
            let (res_cpu, rf): (u16, u64);
            unsafe {
                asm!("rol {a:x}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.rol(v as u64, c as u64, 16) as u16;
            assert_eq!(res_cpu, res_m, "rol16({v:#x},{c}) result");
            check_rot("rol16", v as u64, c, 0x1f, rf, &mut f);
        }
    }
}

#[test]
fn ror16_matches_cpu() {
    for &v in VALS16 {
        for &c in COUNTS {
            let (res_cpu, rf): (u16, u64);
            unsafe {
                asm!("ror {a:x}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.ror(v as u64, c as u64, 16) as u16;
            assert_eq!(res_cpu, res_m, "ror16({v:#x},{c}) result");
            check_rot("ror16", v as u64, c, 0x1f, rf, &mut f);
        }
    }
}

#[test]
fn rcl16_matches_cpu() {
    for &cin in &[false, true] {
        for &v in VALS16 {
            for &c in COUNTS {
                let (res_cpu, rf): (u16, u64);
                unsafe {
                    asm!("bt {ci}, 0", "rcl {a:x}, cl", "pushfq", "pop {rf}",
                        ci = in(reg) cin as u64,
                        a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = f.rcl(v as u64, c as u64, 16) as u16;
                assert_eq!(res_cpu, res_m, "rcl16({v:#x},{c},cf={cin}) result");
                check_rot("rcl16", v as u64, c, 0x1f, rf, &mut f);
            }
        }
    }
}

#[test]
fn rcr16_matches_cpu() {
    for &cin in &[false, true] {
        for &v in VALS16 {
            for &c in COUNTS {
                let (res_cpu, rf): (u16, u64);
                unsafe {
                    asm!("bt {ci}, 0", "rcr {a:x}, cl", "pushfq", "pop {rf}",
                        ci = in(reg) cin as u64,
                        a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = f.rcr(v as u64, c as u64, 16) as u16;
                assert_eq!(res_cpu, res_m, "rcr16({v:#x},{c},cf={cin}) result");
                check_rot("rcr16", v as u64, c, 0x1f, rf, &mut f);
            }
        }
    }
}

const VALS32: &[u32] = &[0, 1, 0x8000_0000, 0x7fff_ffff, 0xffff_ffff, 0xaaaa_aaaa, 0xdead_beef];

#[test]
fn rol32_matches_cpu() {
    for &v in VALS32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("rol {a:e}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.rol(v as u64, c as u64, 32) as u32;
            assert_eq!(res_cpu, res_m, "rol32({v:#x},{c}) result");
            check_rot("rol32", v as u64, c, 0x1f, rf, &mut f);
        }
    }
}

#[test]
fn ror32_matches_cpu() {
    for &v in VALS32 {
        for &c in COUNTS {
            let (res_cpu, rf): (u32, u64);
            unsafe {
                asm!("ror {a:e}, cl", "pushfq", "pop {rf}",
                    a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
            }
            let mut f = Flags::new();
            let res_m = f.ror(v as u64, c as u64, 32) as u32;
            assert_eq!(res_cpu, res_m, "ror32({v:#x},{c}) result");
            check_rot("ror32", v as u64, c, 0x1f, rf, &mut f);
        }
    }
}

// ---- RCL / RCR (rotate through carry) ----

#[test]
fn rcl8_matches_cpu() {
    for &cin in &[false, true] {
        for &v in VALS8 {
            for &c in COUNTS {
                let (res_cpu, rf): (u8, u64);
                unsafe {
                    asm!("bt {ci}, 0", "rcl {a}, cl", "pushfq", "pop {rf}",
                        ci = in(reg) cin as u64,
                        a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = f.rcl(v as u64, c as u64, 8) as u8;
                assert_eq!(res_cpu, res_m, "rcl8({v:#x},{c},cf={cin}) result");
                check_rot("rcl8", v as u64, c, 0x1f, rf, &mut f);
            }
        }
    }
}

#[test]
fn rcr8_matches_cpu() {
    for &cin in &[false, true] {
        for &v in VALS8 {
            for &c in COUNTS {
                let (res_cpu, rf): (u8, u64);
                unsafe {
                    asm!("bt {ci}, 0", "rcr {a}, cl", "pushfq", "pop {rf}",
                        ci = in(reg) cin as u64,
                        a = inout(reg_byte) v => res_cpu, in("cl") c, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = f.rcr(v as u64, c as u64, 8) as u8;
                assert_eq!(res_cpu, res_m, "rcr8({v:#x},{c},cf={cin}) result");
                check_rot("rcr8", v as u64, c, 0x1f, rf, &mut f);
            }
        }
    }
}

#[test]
fn rcl32_matches_cpu() {
    for &cin in &[false, true] {
        for &v in VALS32 {
            for &c in COUNTS {
                let (res_cpu, rf): (u32, u64);
                unsafe {
                    asm!("bt {ci}, 0", "rcl {a:e}, cl", "pushfq", "pop {rf}",
                        ci = in(reg) cin as u64,
                        a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = f.rcl(v as u64, c as u64, 32) as u32;
                assert_eq!(res_cpu, res_m, "rcl32({v:#x},{c},cf={cin}) result");
                check_rot("rcl32", v as u64, c, 0x1f, rf, &mut f);
            }
        }
    }
}

#[test]
fn rcr32_matches_cpu() {
    for &cin in &[false, true] {
        for &v in VALS32 {
            for &c in COUNTS {
                let (res_cpu, rf): (u32, u64);
                unsafe {
                    asm!("bt {ci}, 0", "rcr {a:e}, cl", "pushfq", "pop {rf}",
                        ci = in(reg) cin as u64,
                        a = inout(reg) v => res_cpu, in("cl") c, rf = out(reg) rf);
                }
                let mut f = Flags::new();
                f.f_cf = cin;
                let res_m = f.rcr(v as u64, c as u64, 32) as u32;
                assert_eq!(res_cpu, res_m, "rcr32({v:#x},{c},cf={cin}) result");
                check_rot("rcr32", v as u64, c, 0x1f, rf, &mut f);
            }
        }
    }
}
