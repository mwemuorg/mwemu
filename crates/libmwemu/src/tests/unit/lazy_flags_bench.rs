use std::hint::black_box;
use std::time::{Duration, Instant};

use crate::emu32;
use crate::flags::{Flags, PARITY_LOOKUP_TABLE};

const ARITH_FLAGS_MASK: u32 = 0x8d5;

#[inline(always)]
fn mask(bits: u32) -> u64 {
    match bits {
        64 => u64::MAX,
        32 => 0xffff_ffff,
        16 => 0xffff,
        8 => 0xff,
        _ => unreachable!("weird size"),
    }
}

#[inline(always)]
fn sign_mask(bits: u32) -> u64 {
    1_u64 << (bits - 1)
}

#[inline(always)]
fn signed_value(value: u64, bits: u32) -> i128 {
    let value = value & mask(bits);
    if (value & sign_mask(bits)) == 0 {
        value as i128
    } else {
        value as i128 - (1_i128 << bits)
    }
}

#[inline(always)]
fn signed_range(bits: u32) -> (i128, i128) {
    let max = (1_i128 << (bits - 1)) - 1;
    (-1_i128 << (bits - 1), max)
}

#[inline(always)]
fn set_szp(flags: &mut Flags, result: u64, bits: u32) {
    let result = result & mask(bits);
    flags.f_sf = (result & sign_mask(bits)) != 0;
    flags.f_zf = result == 0;
    flags.f_pf = PARITY_LOOKUP_TABLE[result as u8 as usize];
}

#[inline(always)]
fn expected_add(flags: &mut Flags, value1: u64, value2: u64, carry: bool, bits: u32) -> u64 {
    flags.clear_lazy();

    let value1 = value1 & mask(bits);
    let value2 = value2 & mask(bits);
    let carry = carry as u64;
    let sum = value1 as u128 + value2 as u128 + carry as u128;
    let result = (sum & mask(bits) as u128) as u64;

    flags.f_cf = sum > mask(bits) as u128;
    flags.f_af = ((value1 ^ value2 ^ result) & 0x10) != 0;
    set_szp(flags, result, bits);

    let signed_sum = signed_value(value1, bits) + signed_value(value2, bits) + carry as i128;
    let (min, max) = signed_range(bits);
    flags.f_of = signed_sum < min || signed_sum > max;

    result
}

#[inline(always)]
fn expected_sub(flags: &mut Flags, value1: u64, value2: u64, borrow: bool, bits: u32) -> u64 {
    flags.clear_lazy();

    let value1 = value1 & mask(bits);
    let value2 = value2 & mask(bits);
    let borrow = borrow as u64;
    let result = value1.wrapping_sub(value2).wrapping_sub(borrow) & mask(bits);

    flags.f_cf = (value1 as u128) < value2 as u128 + borrow as u128;
    flags.f_af = ((value1 ^ value2 ^ result) & 0x10) != 0;
    set_szp(flags, result, bits);

    let signed_diff = signed_value(value1, bits) - signed_value(value2, bits) - borrow as i128;
    let (min, max) = signed_range(bits);
    flags.f_of = signed_diff < min || signed_diff > max;

    result
}

#[inline(always)]
fn expected_logic(flags: &mut Flags, result: u64, bits: u32) {
    flags.clear_lazy();
    flags.f_cf = false;
    flags.f_af = false;
    flags.f_of = false;
    set_szp(flags, result, bits);
}

#[inline(always)]
fn expected_inc(flags: &mut Flags, value: u64, bits: u32) -> u64 {
    flags.clear_lazy();
    let value = value & mask(bits);
    let result = value.wrapping_add(1) & mask(bits);

    flags.f_af = (value & 0xf) == 0xf;
    flags.f_of = value == sign_mask(bits) - 1;
    set_szp(flags, result, bits);

    result
}

#[inline(always)]
fn expected_dec(flags: &mut Flags, value: u64, bits: u32) -> u64 {
    flags.clear_lazy();
    let value = value & mask(bits);
    let result = value.wrapping_sub(1) & mask(bits);

    flags.f_af = (value & 0xf) == 0;
    flags.f_of = value == sign_mask(bits);
    set_szp(flags, result, bits);

    result
}

#[inline(always)]
fn lazy_add(flags: &mut Flags, value1: u64, value2: u64, carry: bool, bits: u32) -> u64 {
    match bits {
        64 => flags.add64(value1, value2, carry, carry),
        32 => flags.add32(value1 as u32, value2 as u32, carry, carry),
        16 => flags.add16(value1 as u16, value2 as u16, carry, carry),
        8 => flags.add8(value1 as u8, value2 as u8, carry, carry),
        _ => unreachable!("weird size"),
    }
}

#[inline(always)]
fn lazy_sub(flags: &mut Flags, value1: u64, value2: u64, borrow: bool, bits: u32) -> u64 {
    if borrow {
        match bits {
            64 => flags.sub64_borrow(value1, value2, true),
            32 => flags.sub32_borrow(value1, value2, true),
            16 => flags.sub16_borrow(value1, value2, true),
            8 => flags.sub8_borrow(value1, value2, true),
            _ => unreachable!("weird size"),
        }
    } else {
        match bits {
            64 => flags.sub64(value1, value2),
            32 => flags.sub32(value1, value2),
            16 => flags.sub16(value1, value2),
            8 => flags.sub8(value1, value2),
            _ => unreachable!("weird size"),
        }
    }
}

#[inline(always)]
fn lazy_inc(flags: &mut Flags, value: u64, bits: u32) -> u64 {
    match bits {
        64 => flags.inc64(value),
        32 => flags.inc32(value),
        16 => flags.inc16(value),
        8 => flags.inc8(value),
        _ => unreachable!("weird size"),
    }
}

#[inline(always)]
fn lazy_dec(flags: &mut Flags, value: u64, bits: u32) -> u64 {
    match bits {
        64 => flags.dec64(value),
        32 => flags.dec32(value),
        16 => flags.dec16(value),
        8 => flags.dec8(value),
        _ => unreachable!("weird size"),
    }
}

#[inline(always)]
fn arithmetic_tuple(flags: &Flags) -> (bool, bool, bool, bool, bool, bool) {
    (
        flags.cf(),
        flags.pf(),
        flags.af(),
        flags.zf(),
        flags.sf(),
        flags.of(),
    )
}

#[inline(never)]
fn run_lazy(iterations: u64) -> (Duration, u64, u32) {
    let mut flags = Flags::new();
    let mut acc = black_box(0x1234_5678_u64);
    let mut branch_score = 0_u64;
    let start = Instant::now();

    for i in 0..iterations {
        let addend = i.wrapping_mul(0x9e37_79b1) ^ acc.rotate_left(7);
        acc = flags.add32(
            (acc ^ addend) as u32,
            addend.wrapping_add(i & 0xff) as u32,
            false,
            false,
        );

        if !flags.cf() && !flags.zf() {
            branch_score ^= acc;
        }
        if flags.sf() == flags.of() {
            branch_score = branch_score.wrapping_add(3);
        }
    }

    let elapsed = start.elapsed();
    black_box((elapsed, acc ^ branch_score, flags.dump()))
}

#[inline(always)]
fn mixed_eager_add32(flags: &mut Flags, value1: u64, value2: u64) -> u64 {
    flags.clear_lazy();

    let value1 = value1 & 0xffff_ffff;
    let value2 = value2 & 0xffff_ffff;
    let sum = value1 + value2;
    let result = sum & 0xffff_ffff;

    flags.f_cf = sum > 0xffff_ffff;
    flags.f_sf = (result as u32 as i32) < 0;
    flags.f_zf = result == 0;
    flags.f_pf = PARITY_LOOKUP_TABLE[result as u8 as usize];
    flags.f_af = ((value1 ^ value2 ^ result) & 0x10) != 0;

    let sign1 = (value1 >> 31) & 1;
    let sign2 = (value2 >> 31) & 1;
    let signr = (result >> 31) & 1;
    flags.f_of = (sign1 == sign2) && (sign1 != signr);

    result
}

#[inline(never)]
fn run_mixed_eager(iterations: u64) -> (Duration, u64, u32) {
    let mut flags = Flags::new();
    let mut acc = black_box(0x1234_5678_u64);
    let mut branch_score = 0_u64;
    let start = Instant::now();

    for i in 0..iterations {
        let addend = i.wrapping_mul(0x9e37_79b1) ^ acc.rotate_left(7);
        acc = mixed_eager_add32(&mut flags, acc ^ addend, addend.wrapping_add(i & 0xff));

        if !flags.f_cf && !flags.f_zf {
            branch_score ^= acc;
        }
        if flags.f_sf == flags.f_of {
            branch_score = branch_score.wrapping_add(3);
        }
    }

    let elapsed = start.elapsed();
    black_box((elapsed, acc ^ branch_score, flags.dump()))
}

#[test]
fn lazy_arithmetic_matches_expected_flags() {
    for bits in [8, 16, 32, 64] {
        let cases = [
            0,
            1,
            0xf,
            0x10,
            sign_mask(bits) - 1,
            sign_mask(bits),
            mask(bits) - 1,
            mask(bits),
        ];

        for lhs in cases {
            for rhs in cases {
                for carry in [false, true] {
                    let mut expected = Flags::new();
                    let mut lazy = Flags::new();
                    let expected_result = expected_add(&mut expected, lhs, rhs, carry, bits);
                    let lazy_result = lazy_add(&mut lazy, lhs, rhs, carry, bits);

                    assert_eq!(
                        lazy_result, expected_result,
                        "add result mismatch bits={bits} {lhs:x}+{rhs:x}+{carry:?}"
                    );
                    assert_eq!(
                        arithmetic_tuple(&lazy),
                        arithmetic_tuple(&expected),
                        "add accessor flags mismatch bits={bits} {lhs:x}+{rhs:x}+{carry:?}"
                    );
                    assert_eq!(
                        lazy.dump() & ARITH_FLAGS_MASK,
                        expected.dump() & ARITH_FLAGS_MASK,
                        "add dump flags mismatch bits={bits} {lhs:x}+{rhs:x}+{carry:?}"
                    );

                    let mut expected = Flags::new();
                    let mut lazy = Flags::new();
                    let expected_result = expected_sub(&mut expected, lhs, rhs, carry, bits);
                    let lazy_result = lazy_sub(&mut lazy, lhs, rhs, carry, bits);

                    assert_eq!(
                        lazy_result, expected_result,
                        "sub result mismatch bits={bits} {lhs:x}-{rhs:x}-{carry:?}"
                    );
                    assert_eq!(
                        arithmetic_tuple(&lazy),
                        arithmetic_tuple(&expected),
                        "sub accessor flags mismatch bits={bits} {lhs:x}-{rhs:x}-{carry:?}"
                    );
                    assert_eq!(
                        lazy.dump() & ARITH_FLAGS_MASK,
                        expected.dump() & ARITH_FLAGS_MASK,
                        "sub dump flags mismatch bits={bits} {lhs:x}-{rhs:x}-{carry:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn lazy_arithmetic_randomized_matches_expected_flags() {
    let mut seed = 0x1234_5678_9abc_def0_u64;

    for bits in [8, 16, 32, 64] {
        for _ in 0..10_000 {
            seed = seed
                .wrapping_mul(0x5851_f42d_4c95_7f2d)
                .wrapping_add(0x1405_7b7e_f767_814f);
            let lhs = seed & mask(bits);
            seed = seed
                .wrapping_mul(0x5851_f42d_4c95_7f2d)
                .wrapping_add(0x1405_7b7e_f767_814f);
            let rhs = seed & mask(bits);

            for carry in [false, true] {
                let mut expected = Flags::new();
                let mut lazy = Flags::new();
                let expected_result = expected_add(&mut expected, lhs, rhs, carry, bits);
                let lazy_result = lazy_add(&mut lazy, lhs, rhs, carry, bits);

                assert_eq!(
                    lazy_result, expected_result,
                    "random add result mismatch bits={bits}"
                );
                assert_eq!(
                    arithmetic_tuple(&lazy),
                    arithmetic_tuple(&expected),
                    "random add flags mismatch bits={bits} lhs={lhs:x} rhs={rhs:x} carry={carry:?}"
                );

                let mut expected = Flags::new();
                let mut lazy = Flags::new();
                let expected_result = expected_sub(&mut expected, lhs, rhs, carry, bits);
                let lazy_result = lazy_sub(&mut lazy, lhs, rhs, carry, bits);

                assert_eq!(
                    lazy_result, expected_result,
                    "random sub result mismatch bits={bits}"
                );
                assert_eq!(
                    arithmetic_tuple(&lazy),
                    arithmetic_tuple(&expected),
                    "random sub flags mismatch bits={bits} lhs={lhs:x} rhs={rhs:x} carry={carry:?}"
                );
            }
        }
    }
}

#[test]
fn calc_pf_and_calc_af_preserve_pending_lazy_flags() {
    let result = 0x8000_0001_u64;

    let mut flags = Flags::new();
    flags.f_cf = true;
    flags.f_of = false;
    flags.f_af = true;
    flags.calc_flags(result, 32);
    flags.calc_pf(result as u8);

    assert!(
        flags.cf(),
        "CF should survive calc_pf after lazy calc_flags"
    );
    assert!(
        flags.sf(),
        "SF should survive calc_pf after lazy calc_flags"
    );
    assert!(
        !flags.zf(),
        "ZF should survive calc_pf after lazy calc_flags"
    );
    assert!(
        !flags.of(),
        "OF should survive calc_pf after lazy calc_flags"
    );

    let mut flags = Flags::new();
    flags.f_cf = true;
    flags.f_of = true;
    flags.calc_flags(0, 32);
    flags.calc_af(0xf, 1, 0x10, 32);

    assert!(
        flags.cf(),
        "CF should survive calc_af after lazy calc_flags"
    );
    assert!(
        flags.zf(),
        "ZF should survive calc_af after lazy calc_flags"
    );
    assert!(
        flags.of(),
        "OF should survive calc_af after lazy calc_flags"
    );
    assert!(flags.af(), "AF should be overwritten by calc_af");
}

#[test]
fn lazy_logic_matches_expected_flags() {
    for bits in [8, 16, 32, 64] {
        for result in [0, 1, 0x7f, 0x80, sign_mask(bits), mask(bits)] {
            let mut expected = Flags::new();
            let mut lazy = Flags::new();

            expected_logic(&mut expected, result, bits);
            lazy.calc_logic_flags_lazy(result, bits);

            assert_eq!(
                arithmetic_tuple(&lazy),
                arithmetic_tuple(&expected),
                "logic accessor flags mismatch bits={bits} result={result:x}"
            );
            assert_eq!(
                lazy.dump() & ARITH_FLAGS_MASK,
                expected.dump() & ARITH_FLAGS_MASK,
                "logic dump flags mismatch bits={bits} result={result:x}"
            );
        }
    }
}

#[test]
fn lazy_inc_dec_matches_expected_flags_and_preserves_cf() {
    for bits in [8, 16, 32, 64] {
        let cases = [
            0,
            1,
            0xf,
            0x10,
            sign_mask(bits) - 1,
            sign_mask(bits),
            mask(bits) - 1,
            mask(bits),
        ];

        for value in cases {
            for initial_cf in [false, true] {
                let mut expected = Flags::new();
                expected.f_cf = initial_cf;
                let expected_result = expected_inc(&mut expected, value, bits);

                let mut lazy = Flags::new();
                lazy.f_cf = initial_cf;
                let lazy_result = lazy_inc(&mut lazy, value, bits);

                assert_eq!(
                    lazy_result, expected_result,
                    "inc result mismatch bits={bits} value={value:x} cf={initial_cf:?}"
                );
                assert_eq!(
                    arithmetic_tuple(&lazy),
                    arithmetic_tuple(&expected),
                    "inc accessor flags mismatch bits={bits} value={value:x} cf={initial_cf:?}"
                );
                assert_eq!(
                    lazy.dump() & ARITH_FLAGS_MASK,
                    expected.dump() & ARITH_FLAGS_MASK,
                    "inc dump flags mismatch bits={bits} value={value:x} cf={initial_cf:?}"
                );

                let mut expected = Flags::new();
                expected.f_cf = initial_cf;
                let expected_result = expected_dec(&mut expected, value, bits);

                let mut lazy = Flags::new();
                lazy.f_cf = initial_cf;
                let lazy_result = lazy_dec(&mut lazy, value, bits);

                assert_eq!(
                    lazy_result, expected_result,
                    "dec result mismatch bits={bits} value={value:x} cf={initial_cf:?}"
                );
                assert_eq!(
                    arithmetic_tuple(&lazy),
                    arithmetic_tuple(&expected),
                    "dec accessor flags mismatch bits={bits} value={value:x} cf={initial_cf:?}"
                );
                assert_eq!(
                    lazy.dump() & ARITH_FLAGS_MASK,
                    expected.dump() & ARITH_FLAGS_MASK,
                    "dec dump flags mismatch bits={bits} value={value:x} cf={initial_cf:?}"
                );
            }
        }
    }
}

#[test]
fn lazy_inc_dec_preserve_cf_from_pending_lazy_width() {
    for bits in [8, 16, 32, 64] {
        for initial_cf in [false, true] {
            let mut flags = Flags::new();
            if initial_cf {
                flags.sub8(0, 1);
            } else {
                flags.add8(1, 1, false, false);
            }

            lazy_inc(&mut flags, 0, bits);
            assert_eq!(
                flags.cf(),
                initial_cf,
                "inc should preserve pending lazy CF bits={bits} cf={initial_cf:?}"
            );

            if initial_cf {
                flags.sub8(0, 1);
            } else {
                flags.add8(1, 1, false, false);
            }

            lazy_dec(&mut flags, 0, bits);
            assert_eq!(
                flags.cf(),
                initial_cf,
                "dec should preserve pending lazy CF bits={bits} cf={initial_cf:?}"
            );
        }
    }
}

#[test]
#[ignore = "performance benchmark; run with --release -- --ignored --nocapture"]
fn lazy_add32_flag_benchmark() {
    let iterations = std::env::var("MWEMU_LAZY_FLAGS_BENCH_ITERS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(20_000_000_u64);

    let (mixed_elapsed, mixed_value, mixed_flags) = run_mixed_eager(iterations);
    let (lazy_elapsed, lazy_value, lazy_flags) = run_lazy(iterations);
    assert_eq!(lazy_value, mixed_value);
    assert_eq!(lazy_flags, mixed_flags);
    let speedup = mixed_elapsed.as_secs_f64() / lazy_elapsed.as_secs_f64();
    println!(
        "lazy_add32_flag_benchmark iterations={iterations} mixed={:?} bochs_lazy={:?} speedup={:.3}x",
        mixed_elapsed, lazy_elapsed, speedup
    );
}

#[test]
#[ignore = "performance benchmark; run with --release -- --ignored --nocapture"]
fn synthetic_x86_flag_loop_benchmark() {
    let iterations = std::env::var("MWEMU_SYNTH_FLAG_LOOP_ITERS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1_000_000_u32);

    let mut code = Vec::with_capacity(21);
    code.extend_from_slice(&[0x31, 0xc0]); // xor eax,eax
    code.push(0xb9); // mov ecx, iterations
    code.extend_from_slice(&iterations.to_le_bytes());
    code.push(0x05); // add eax, imm32
    code.extend_from_slice(&0x0000_3039_u32.to_le_bytes());
    code.push(0x3d); // cmp eax, imm32
    code.extend_from_slice(&0xdead_beef_u32.to_le_bytes());
    code.push(0x49); // dec ecx
    code.extend_from_slice(&[0x75, 0xf3]); // jnz back to add
    code.push(0xc3); // ret, not reached by run_to target

    let mut emu = emu32();
    emu.cfg.console = false;
    emu.cfg.console_enabled = false;
    emu.disable_ctrlc();
    emu.load_code_bytes(&code);

    let target_instructions = 2_u64 + (iterations as u64 * 4);
    let start = Instant::now();
    emu.run_to(target_instructions)
        .expect("synthetic flag loop should execute");
    let elapsed = start.elapsed();

    assert_eq!(emu.regs().get_ecx(), 0);
    let ips = target_instructions as f64 / elapsed.as_secs_f64();
    println!(
        "synthetic_x86_flag_loop_benchmark iterations={iterations} instructions={target_instructions} elapsed={:?} ips={:.0}",
        elapsed, ips
    );
}
