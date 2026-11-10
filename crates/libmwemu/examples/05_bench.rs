//! Measure how fast the emulator runs three small x86-64 loops.
//!
//! This is the yardstick for changes that could slow the run loop down, such
//! as moving code between crates: run it before and after, in release mode,
//! and compare the instructions per second.
//!
//! Run with:
//! ```sh
//! cargo run --release -p libmwemu --features examples --example 05_bench
//! ```
//!
//! Single runs vary by a few percent, so run it several times and take the
//! median.

use libmwemu::emu::Emu;
use libmwemu::emu64;
use libmwemu::maps::mem64::Permission;
use std::time::{Duration, Instant};

/// Iterations of the two hot loops.
const ITERATIONS: u32 = 20_000_000;
/// One-instruction blocks in the cache-miss case, and how often they are run.
const BLOCKS: usize = 300_000;
const BLOCK_PASSES: usize = 40;

fn emu_with(code: &[u8]) -> Emu {
    let mut emu = emu64();
    emu.load_code_bytes(code).expect("the code maps");
    emu
}

fn report(name: &str, emu: &Emu, elapsed: Duration) {
    println!(
        "{name:18} {:>10} instructions in {:.3} s  {:6.2} M instr/s",
        emu.pos,
        elapsed.as_secs_f64(),
        emu.pos as f64 / elapsed.as_secs_f64() / 1e6
    );
}

fn main() {
    // Registers only, one cached block:
    //   mov rcx, N ; L: add rax, rbx ; xor rdx, rax ; dec rcx ; jnz L ; hlt
    let mut code = vec![0x48, 0xc7, 0xc1];
    code.extend_from_slice(&ITERATIONS.to_le_bytes());
    code.extend_from_slice(&[
        0x48, 0x01, 0xd8, // add rax, rbx
        0x48, 0x31, 0xc2, // xor rdx, rax
        0x48, 0xff, 0xc9, // dec rcx
        0x75, 0xf5, // jnz L
        0xf4, // hlt
    ]);
    let mut emu = emu_with(&code);
    let start = Instant::now();
    let _ = emu.run(None);
    report("register loop", &emu, start.elapsed());

    // The same with one store and one load per iteration:
    //   mov rcx, N ; L: mov [rsi], rax ; mov rbx, [rsi] ; add rax, rbx ; dec rcx ; jnz L ; hlt
    let mut code = vec![0x48, 0xc7, 0xc1];
    code.extend_from_slice(&ITERATIONS.to_le_bytes());
    code.extend_from_slice(&[
        0x48, 0x89, 0x06, // mov [rsi], rax
        0x48, 0x8b, 0x1e, // mov rbx, [rsi]
        0x48, 0x01, 0xd8, // add rax, rbx
        0x48, 0xff, 0xc9, // dec rcx
        0x75, 0xf2, // jnz L
        0xf4, // hlt
    ]);
    let mut emu = emu_with(&code);
    emu.maps
        .create_map("bench_data", 0x1000_0000, 0x1000, Permission::READ_WRITE)
        .expect("the data page maps");
    emu.regs_mut().rsi = 0x1000_0000;
    let start = Instant::now();
    let _ = emu.run(None);
    report("memory loop", &emu, start.elapsed());

    // Every block misses the instruction cache: `jmp +0` many times, then hlt.
    let mut code = Vec::with_capacity(BLOCKS * 2 + 1);
    for _ in 0..BLOCKS {
        code.extend_from_slice(&[0xeb, 0x00]);
    }
    code.push(0xf4);
    let mut emu = emu_with(&code);
    let entry = emu.pc();
    let start = Instant::now();
    for _ in 0..BLOCK_PASSES {
        emu.set_pc(entry);
        let _ = emu.run(None);
    }
    report("cache-miss blocks", &emu, start.elapsed());
}
