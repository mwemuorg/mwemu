//! Emulate a handful of raw x86-64 instructions and read the result back.
//!
//! The smallest useful thing libmwemu does: no file on disk, no OS, no imports
//! — just bytes in, register state out.
//!
//! Run with:
//! ```sh
//! cargo run -p libmwemu --features examples --example 01_shellcode
//! ```

use libmwemu::emu64;

fn main() {
    // `emu64()` builds an x86-64 emulator; `emu32()` and `emu_aarch64()` are the
    // other two entry points.
    let mut emu = emu64();

    // mov rax, 5
    // mov rbx, 37
    // add rax, rbx
    // imul rax, rax, 2
    let code: &[u8] = &[
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // mov rax, 5
        0x48, 0xc7, 0xc3, 0x25, 0x00, 0x00, 0x00, // mov rbx, 37
        0x48, 0x01, 0xd8, // add rax, rbx
        0x48, 0x6b, 0xc0, 0x02, // imul rax, rax, 2
    ];

    // Maps the bytes as executable code and points the instruction pointer at
    // them. Registers and flags start zeroed.
    emu.load_code_bytes(code);

    // `step()` runs exactly one instruction and returns false when execution
    // cannot continue. Use it when you want to watch the machine evolve; see
    // 04_load_binary.rs for `run()`, which goes until it stops or hits a limit.
    for _ in 0..4 {
        if !emu.step() {
            break;
        }
    }

    let rax = emu.regs().rax;
    println!("rax = {} (0x{:x})", rax, rax);
    assert_eq!(rax, (5 + 37) * 2);

    println!("rbx = {}", emu.regs().rbx);
    println!("zero flag = {}", emu.flags().f_zf);
}
