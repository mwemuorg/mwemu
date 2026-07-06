
# LIBMWEMU x86 emulation + win32 simulation 
[![Rust CI](https://github.com/mwemuorg/libmwemu/actions/workflows/ci.yaml/badge.svg)](https://github.com/mwemuorg/libmwemu/actions/workflows/ci.yaml)
[![libmwemu crates.io](https://img.shields.io/crates/v/libmwemu.svg)](https://crates.io/crates/libmwemu)
[![Docs.rs](https://docs.rs/libmwemu/badge.svg)](https://docs.rs/libmwemu)

## Documentation

Check the official documentation, here there are explanations, examples, and links to docs.rs:

- https://mwemu.github.io/#libmwemu

The docs.rs automatic documentation:

- https://docs.rs/libmwemu/0.23.5/libmwemu/

## Crate

https://crates.io/crates/libmwemu

## Github

https://github.com/mwemuorg/mwemu

## Usage

Download the maps from:
https://github.com/mwemuorg/mwemu

Create an emu32 or emu64 and it's important to set the maps folder.

```rust
use libmwemu::emu32;


fn main() {
    let mut emu = emu32();
    emu.set_maps_folder("/tmp/maps/windows/x86/");
    emu.init_logger();
```

Load your shellcode or PE binary and run the emulator.
None parameter means emulate for-ever.

```rust
// emu.init(false, false); needed if load_code is not called
emu.load_code("shellcodes32/shikata.bin");
emu.set_verbose(2);
emu.run(None).unwrap(); 
```

Or if you prefer call specific function.

```rust
emu.load_code("samples/malware.exe");

let crypto_key_gen = 0x40112233;
let ret_addr = 0x40110000; // any place safe to return.

let param1 = 0x33;
let param2_out_buff = emu.alloc("buffer", 1024, Permission::READ_WRITE_EXECUTE);

emu.maps.memset(param2_out_buff, 0, 1024); // non necesary, by default alloc create zeros.
emu.maps.write_spaced_bytes(param2_out_buff, 
        "DE CC 6C 83 CC F3 66 85 34"); // example of initialization.

// call function
emu.regs.set_eip(crypto_key_gen);
emu.stack_push32(param2_out_buff);
emu.stack_push32(param1);
emu.stack_push32(ret_addr);
emu.run(Some(ret_addr)).unwrap();   // emulate until arrive to ret_addr

// or simpler way:
let eax = emu.call32(crypto_key_gen, &[param1, param2_out_buff]).unwrap();

// this would be slower but more control
while emu.step() {
    ...
}

// check result
log::info!("return value: 0x{:x}", emu.regs.get_eax());
emu.maps.dump(param2_out_buff);
```

Now it's possible to do hooks on libmwemu but not on pymwemu.

```rust
use libmwemu::emu32;

//need iced_x86 crate only for instruction hooks, to get the
//instruction object, so add `iced-x86 = "1.17.0"`
use iced_x86::{Instruction};  


fn trace_memory_read(emu:&mut libmwemu::emu::Emu, ip_addr:u64, 
                     mem_addr:u64, sz:u8) {
    log::info!("0x{:x}: reading {} at 0x{:x}", ip_addr, sz, mem_addr);
    if mem_addr == 0x22dff0 {
        emu.stop();
    }
}

fn trace_memory_write(emu:&mut libmwemu::emu::Emu, ip_addr:u64, 
                      mem_addr:u64, sz:u8, value:u128) -> u128 {
    log::info!("0x{:x}: writing {} '0x{:x}' at 0x{:x}", ip_addr, sz, 
             value, mem_addr);
    value   // I could change the value to write
}

fn trace_interrupt(emu:&mut libmwemu::emu::Emu, ip_addr:u64, 
                   interrupt:u64) -> bool {
    log::info!("interrupt {} triggered at eip: 0x{:x}", interrupt, 
             ip_addr);
    true  // do handle interrupts
}   

// [BREAKING API CHANGE] a parameter called ex_type with the exception type was added to the hook function.
fn trace_exceptions(emu:&mut libmwemu::emu::Emu, ip_addr:u64, ex_type: libmwemu::exception_type::ExceptionType) -> bool {
    log::info!("0x{:x} triggered an exception {}", ip_addr, ex_type);
    if (ex_type == libmwemu::exception_type::ExceptionType::Int3) {
        // do handle SIGTRAP for example
    }
    true // do handle exceptions
}

fn trace_pre_instruction(emu:&mut libmwemu::emu::Emu, ip_addr:u64, 
                         ins:&Instruction, sz:usize) -> bool{
                            // return false to skip the instruction
                            true
}

fn trace_post_instruction(emu:&mut libmwemu::emu::Emu, ip_addr:u64, 
                          ins:&Instruction, sz:usize, emu_ok:bool) {
}

fn trace_winapi_call(emu:&mut libmwemu::emu::Emu, ip_addr:u64, api_addr:u64) -> bool {
    return true; // handle api calls
}

fn main() {
    let mut emu = emu32();
    emu.set_maps_folder("../mwemu/maps/windows/x86/"); // download the maps, ideally from mwemu git.
    emu.init();

    emu.load_code("/home/sha0/src/mwemu/shellcodes32/mars.exe");
    emu.hooks.on_memory_read(trace_memory_read);
    emu.hooks.on_memory_write(trace_memory_write);
    emu.hooks.on_interrupt(trace_interrupt);
    emu.hooks.on_exception(trace_exceptions);
    emu.hooks.on_pre_instruction(trace_pre_instruction);
    emu.hooks.on_post_instruction(trace_post_instruction);
    emu.hooks.on_winapi_call(trace_winapi_call);
    emu.run(None).unwrap();
    log::info!("end!");
}
```
