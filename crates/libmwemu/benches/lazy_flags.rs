use std::hint::black_box;
use std::time::{Duration, Instant};

use libmwemu::emu32;
use libmwemu::flags::{Flags, PARITY_LOOKUP_TABLE};

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
    emu.load_code_bytes(&code).expect("load benchmark loop");

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

fn main() {
    lazy_add32_flag_benchmark();
    synthetic_x86_flag_loop_benchmark();
}
