//! Observe execution from the outside using hooks.
//!
//! Hooks are how you instrument a sample without patching it: count
//! instructions, log every memory write, watch API calls, or veto an operation
//! by returning `false`.
//!
//! Run with:
//! ```sh
//! cargo run -p libmwemu --features examples --example 03_hooks
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use libmwemu::emu64;
use libmwemu::maps::mem64::Permission;

fn main() {
    let mut emu = emu64();

    let base = 0x4000_0000;
    emu.maps
        .create_map("scratch", base, 0x1000, Permission::READ_WRITE)
        .expect("cannot create map");

    // mov rax, 0x40000000
    // mov qword ptr [rax], 0xcafe
    // mov rcx, qword ptr [rax]
    // xor rdx, rdx
    let code: &[u8] = &[
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x40, // mov rax, 0x40000000
        0x48, 0xc7, 0x00, 0xfe, 0xca, 0x00, 0x00, // mov qword ptr [rax], 0xcafe
        0x48, 0x8b, 0x08, // mov rcx, qword ptr [rax]
        0x48, 0x31, 0xd2, // xor rdx, rdx
    ];

    // Hooks are `'static` closures, so anything they mutate has to outlive the
    // call — an `Rc<RefCell<_>>` is the usual way to keep a counter.
    let executed = Rc::new(RefCell::new(0u64));

    {
        let executed = Rc::clone(&executed);
        // Returning false from a pre-instruction hook skips the instruction.
        emu.hooks.on_pre_instruction(move |_emu, addr, _ins, sz| {
            *executed.borrow_mut() += 1;
            println!("  pre  0x{:x} ({} bytes)", addr, sz);
            true
        });
    }

    // The write hook returns the value that actually gets stored, so it can
    // rewrite it in flight — return `value` unchanged to only observe.
    emu.hooks
        .on_memory_write(move |_emu, rip, mem_addr, bits, value| {
            println!(
                "  write [0x{:x}] <- 0x{:x} ({} bits, from rip 0x{:x})",
                mem_addr, value, bits, rip
            );
            value
        });

    // The read hook fires before the value is fetched, so it gets the address
    // rather than the result.
    emu.hooks.on_memory_read(move |_emu, rip, mem_addr, bits| {
        println!(
            "  read  [0x{:x}] ({} bits, from rip 0x{:x})",
            mem_addr, bits, rip
        );
    });

    emu.load_code_bytes(code);

    println!("running:");
    for _ in 0..4 {
        if !emu.step() {
            break;
        }
    }

    println!("\ninstructions executed: {}", executed.borrow());
    println!("rcx = 0x{:x}", emu.regs().rcx);

    // Hooks can be removed again when you no longer need the overhead.
    emu.hooks.disable_pre_instruction();
}
