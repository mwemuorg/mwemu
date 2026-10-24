#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::assertions_on_constants)]
// NOTE: `neg_cmp_op_on_partial_ord` is kept deliberately — in the float
// compare handlers (cmpss/cmpsd/cmpps/cmppd/avx) `!(a < b)` is NOT the same
// as `a >= b` for NaN operands, which is the intended x86 semantics.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
// The `construct_uint!` macro from `uint` generates code with manual div_ceil;
// we cannot fix it inside the macro expansion.
#![allow(clippy::manual_div_ceil)]

#[macro_use]
pub mod utils;

// Grouped modules
pub mod api;
pub mod arch;
pub mod debug;
pub mod exception;
pub mod kernel;
pub mod loaders;
pub mod threading;
pub mod windows;

// Core modules
pub mod emu;
pub mod engine;
pub mod maps;
pub mod serialization;
pub mod syscall;

// Standalone modules
pub mod config;
pub mod err;
pub mod hooks;

// Backwards-compatible re-exports (arch)
pub use arch::aarch64::regs as regs_aarch64;
pub use arch::x86::context;
pub use arch::x86::eflags;
pub use arch::x86::flags;
pub use arch::x86::fpu;
pub use arch::x86::regs as regs64;

// Backwards-compatible re-exports (api/syscall)
pub use api::linux as linuxapi;
pub use api::macos as macosapi;
pub use api::windows as winapi;
pub use syscall::windows::ntapi;

// Re-exports for external crates
pub use debug::console;
pub use debug::gdb;
pub use debug::script;
pub use emu::emu_context;
pub use utils::color_enabled;
pub use utils::disable_color;

#[cfg(test)]
mod tests;
use arch::Arch;
use emu::Emu;

/// Create an x86-64 emulator with Ctrl-C handling disabled.
pub fn emu64() -> Emu {
    let mut emu = Emu::new(Arch::X86_64);
    emu.disable_ctrlc();
    emu
}

/// Create an x86 (32-bit) emulator with Ctrl-C handling disabled.
pub fn emu32() -> Emu {
    let mut emu = Emu::new(Arch::X86);
    emu.disable_ctrlc();
    emu
}

/// Create an AArch64 emulator with Ctrl-C handling disabled.
pub fn emu_aarch64() -> Emu {
    let mut emu = Emu::new(Arch::Aarch64);
    emu.disable_ctrlc();
    emu
}
