
# LIBMWEMU x86/x64/AArch64 emulation + OS simulation
[![Rust CI](https://github.com/mwemuorg/mwemu/actions/workflows/ci.yaml/badge.svg)](https://github.com/mwemuorg/mwemu/actions/workflows/ci.yaml)
[![libmwemu crates.io](https://img.shields.io/crates/v/libmwemu.svg)](https://crates.io/crates/libmwemu)
[![Docs.rs](https://docs.rs/libmwemu/badge.svg)](https://docs.rs/libmwemu)

## Documentation

Check the official documentation, here there are explanations, examples, and links to docs.rs:

- https://mwemu.github.io/#libmwemu

The docs.rs automatic documentation:

- https://docs.rs/libmwemu/latest/libmwemu/

## Crate

https://crates.io/crates/libmwemu

## Github

https://github.com/mwemuorg/mwemu

## Usage

Download the maps from:
https://github.com/mwemuorg/mwemu

Create an emu32 or emu64 and set the maps folder.

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
emu.load_code("shellcodes32/shikata.bin");
emu.set_verbose(2);
emu.run(None).unwrap(); 
```

Or if you prefer call a specific function.

```rust
use libmwemu::maps::mem64::Permission;

emu.load_code("samples/malware.exe");

let crypto_key_gen = 0x40112233;
let ret_addr = 0x40110000; // any place safe to return.

let param1 = 0x33;
let param2_out_buff = emu.alloc("buffer", 1024, Permission::READ_WRITE_EXECUTE);

emu.maps.memset(param2_out_buff, 0, 1024);
emu.maps.write_spaced_bytes(param2_out_buff, 
        "DE CC 6C 83 CC F3 66 85 34");

// call function
emu.regs_mut().set_eip(crypto_key_gen);
emu.stack_push32(param2_out_buff);
emu.stack_push32(param1);
emu.stack_push32(ret_addr);
emu.run(Some(ret_addr)).unwrap();

// or simpler way:
let eax = emu.call32(crypto_key_gen, &[param1 as u32, param2_out_buff as u32]).unwrap();

// this would be slower but more control
while emu.step() {
    // ...
}

// check result
log::info!("return value: 0x{:x}", emu.regs().get_eax());
emu.maps.dump(param2_out_buff);
```

## Kernel-mode drivers

Load and emulate Windows `.sys` or Linux `.ko` kernel modules:

```rust
use libmwemu::emu64;

let mut emu = emu64();
emu.cfg.verbose = 0;
emu.cfg.skip_unimplemented = true;
emu.load_kernel_module("driver.sys").unwrap();
emu.run_module_init().unwrap();

// send an IOCTL
let result = emu.kernel_ioctl(0, 0x80002000, &input_bytes, 256).unwrap();

// or scan all IOCTL codes automatically
let probes = emu.kernel_scan_ioctls(0, 0x1000);
```

## Hooks

libmwemu provides hooks for intercepting emulation events.

```rust
use libmwemu::emu32;
use libmwemu::emu::decoded_instruction::DecodedInstruction;
use libmwemu::exception::types::ExceptionType;

fn trace_memory_read(emu: &mut libmwemu::emu::Emu, ip_addr: u64,
                     mem_addr: u64, sz: u32) {
    log::info!("0x{:x}: reading {} bits at 0x{:x}", ip_addr, sz, mem_addr);
    if mem_addr == 0x22dff0 {
        emu.stop();
    }
}

fn trace_memory_write(emu: &mut libmwemu::emu::Emu, ip_addr: u64,
                      mem_addr: u64, sz: u32, value: u128) -> u128 {
    log::info!("0x{:x}: writing {} bits '0x{:x}' at 0x{:x}", ip_addr, sz,
             value, mem_addr);
    value   // return value to actually write (can be modified)
}

fn trace_interrupt(emu: &mut libmwemu::emu::Emu, interrupt: u64,
                   ip_addr: u64) -> bool {
    log::info!("interrupt {} triggered at ip: 0x{:x}", interrupt, ip_addr);
    true  // do handle interrupts
}

fn trace_exceptions(emu: &mut libmwemu::emu::Emu, ip_addr: u64,
                    ex_type: ExceptionType) -> bool {
    log::info!("0x{:x} triggered exception {}", ip_addr, ex_type);
    true // do handle exceptions
}

fn trace_pre_instruction(emu: &mut libmwemu::emu::Emu, ip_addr: u64,
                         ins: &DecodedInstruction, sz: usize) -> bool {
    // return false to skip the instruction
    true
}

fn trace_post_instruction(emu: &mut libmwemu::emu::Emu, ip_addr: u64,
                          ins: &DecodedInstruction, sz: usize, emu_ok: bool) {
}

fn trace_winapi_call(emu: &mut libmwemu::emu::Emu, ip_addr: u64,
                     api_addr: u64) -> bool {
    true // handle api calls
}

fn trace_syscall(emu: &mut libmwemu::emu::Emu, syscall_nr: u64) -> bool {
    log::info!("syscall {}", syscall_nr);
    true // return false to skip default syscall handler
}

fn main() {
    let mut emu = emu32();
    emu.set_maps_folder("../mwemu/maps/windows/x86/");
    emu.init_logger();

    emu.load_code("shellcodes32/mars.exe").unwrap();
    emu.hooks.on_memory_read(trace_memory_read);
    emu.hooks.on_memory_write(trace_memory_write);
    emu.hooks.on_interrupt(trace_interrupt);
    emu.hooks.on_exception(trace_exceptions);
    emu.hooks.on_pre_instruction(trace_pre_instruction);
    emu.hooks.on_post_instruction(trace_post_instruction);
    emu.hooks.on_winapi_call(trace_winapi_call);
    emu.hooks.on_syscall(trace_syscall);
    emu.run(None).unwrap();
    log::info!("end!");
}
```
