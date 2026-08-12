//! Load a real executable from disk and emulate it under a step budget.
//!
//! `load_code` sniffs the format (ELF32/64, Mach-O, PE32/64, or raw shellcode)
//! and sets up the address space, entry point and stack accordingly, so the
//! same three lines work for a Linux ELF and a Windows PE.
//!
//! Run with:
//! ```sh
//! cargo run -p libmwemu --features examples --example 04_load_binary -- /bin/ls
//! cargo run -p libmwemu --features examples --example 04_load_binary -- test/exe64win_msgbox.bin 200000
//! ```

use libmwemu::emu64;

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(path) = args.next() else {
        eprintln!("usage: 04_load_binary <binary> [max_instructions]");
        std::process::exit(1);
    };

    // Emulating an unbounded binary is an easy way to hang forever, so every
    // example run is capped. 200k instructions is enough to get through a small
    // ELF; a packed PE needs orders of magnitude more.
    let budget: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(200_000);

    let mut emu = emu64();

    // 0 = APIs only, 1 = + messages, 2 = + disassembly. Bump this to watch the
    // emulation instruction by instruction.
    emu.cfg.verbose = 1;

    // Windows samples resolve imports against these DLLs. Harmless for ELF.
    emu.cfg.maps_folder = "maps/windows/x86_64/".to_string();

    emu.load_code(&path);

    // `run_to` stops after N emulated instructions. Use `run(Some(addr))` to
    // stop when a given address is reached instead, or `run(None)` to go until
    // the program exits on its own.
    //
    // Both return the program counter where emulation stopped — not a count.
    // The instruction count is `emu.pos`.
    match emu.run_to(budget) {
        Ok(pc) => println!("\nstopped cleanly at 0x{:x}", pc),
        Err(e) => println!("\nstopped: {}", e),
    }

    println!("rip = 0x{:x}", emu.regs().rip);
    println!("rax = 0x{:x}", emu.regs().rax);
    println!("instructions emulated: {}", emu.pos);
}
