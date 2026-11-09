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

// Internal modules — not part of the public API.
pub(crate) mod api;
pub(crate) mod engine;
pub(crate) mod exception;
pub(crate) mod loaders;
pub(crate) mod syscall;
pub(crate) mod threading;
pub(crate) mod windows;

// Semi-public modules — exposed for advanced use (snapshot, debugger).
pub mod arch;
pub mod debug;
pub mod kernel;

// Core public modules
pub mod emu;
pub mod maps;
pub mod serialization;

// Standalone public modules
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

// Internal re-exports used by crate modules (api/syscall).
pub(crate) use api::linux as linuxapi;
pub(crate) use api::macos as macosapi;
pub(crate) use api::windows as winapi;
pub(crate) use syscall::windows::ntapi;

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

/// Register the Windows API handlers with the emulator core. `emu32()`,
/// `emu64()` and `emu_aarch64()` do it for you; call it yourself before
/// running an emulator built another way (`Emu::new`, `Serialization::load`).
/// Calling it more than once is harmless.
pub fn register_api_handlers() {
    api::dispatch::register(api::dispatch::ApiDispatch {
        winapi32_gateway: winapi::winapi32::gateway,
        winapi64_gateway: winapi::winapi64::gateway,
        winapi64_gateway_by_import: winapi::winapi64::gateway_by_import,
        winapi64_load_library_a: winapi::winapi64::kernel32::LoadLibraryA,
        winapi64_get_proc_address: winapi::winapi64::kernel32::GetProcAddress,
    });
}

/// Create an emulator for `arch` with Ctrl-C handling disabled.
fn new_emu(arch: Arch) -> Emu {
    register_api_handlers();
    let mut emu = Emu::new(arch);
    emu.disable_ctrlc();
    emu
}

/// Create an x86-64 emulator with Ctrl-C handling disabled.
pub fn emu64() -> Emu {
    new_emu(Arch::X86_64)
}

/// Create an x86 (32-bit) emulator with Ctrl-C handling disabled.
pub fn emu32() -> Emu {
    new_emu(Arch::X86)
}

/// Create an AArch64 emulator with Ctrl-C handling disabled.
pub fn emu_aarch64() -> Emu {
    new_emu(Arch::Aarch64)
}
