//! libmwemu: an x86, x86-64 and AArch64 emulator for malware analysis.
//!
//! This crate is the public face of three internal ones: the emulator core
//! (`mwemu-core`) and the Windows API handlers (`mwemu-winapi32`,
//! `mwemu-winapi64`). It re-exports the core's API and plugs the handlers in.

// Lints the test suite was written under.
#![cfg_attr(test, allow(non_snake_case))]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, allow(unused_variables))]
#![cfg_attr(test, allow(unused_must_use))]
#![cfg_attr(test, allow(clippy::assertions_on_constants))]
// NOTE: `neg_cmp_op_on_partial_ord` is kept deliberately — in the float
// compare handlers (cmpss/cmpsd/cmpps/cmppd/avx) `!(a < b)` is NOT the same
// as `a >= b` for NaN operands, which is the intended x86 semantics.
#![cfg_attr(test, allow(clippy::neg_cmp_op_on_partial_ord))]
// The `construct_uint!` macro from `uint` generates code with manual div_ceil;
// we cannot fix it inside the macro expansion.
#![cfg_attr(test, allow(clippy::manual_div_ceil))]

pub use mwemu_core::{arch, config, debug, emu, err, hooks, kernel, maps, serialization, utils};

// Backwards-compatible re-exports (arch)
pub use mwemu_core::{context, eflags, flags, fpu, regs_aarch64, regs64};

// Re-exports for external crates
pub use mwemu_core::{color_enabled, console, disable_color, emu_context, gdb, script};

// Macros
pub use mwemu_core::{color, get_bit, log_orange, log_red, popn, set_bit, to32};

// The tests were written inside the single-crate libmwemu and reach its
// internals as `crate::…`; keep those paths valid for them.
#[cfg(test)]
pub(crate) use mwemu_core::{engine, exception, syscall, threading, windows};
#[cfg(test)]
pub(crate) mod api {
    pub(crate) use mwemu_core::api::*;
    pub(crate) mod windows {
        pub(crate) use mwemu_core::api::windows::*;
        pub(crate) use mwemu_winapi32 as winapi32;
        pub(crate) use mwemu_winapi64 as winapi64;
    }
}
#[cfg(test)]
pub(crate) use api::windows as winapi;

#[cfg(test)]
mod tests;

use arch::Arch;
use emu::Emu;
use mwemu_core::api::dispatch::{self, ApiDispatch};

/// Register the Windows API handlers with the emulator core. `emu32()`,
/// `emu64()` and `emu_aarch64()` do it for you; call it yourself before
/// running an emulator built another way (`Emu::new`, `Serialization::load`).
/// Calling it more than once is harmless.
pub fn register_api_handlers() {
    dispatch::register(ApiDispatch {
        winapi32_gateway: mwemu_winapi32::gateway,
        winapi64_gateway: mwemu_winapi64::gateway,
        winapi64_gateway_by_import: mwemu_winapi64::gateway_by_import,
        winapi64_load_library_a: mwemu_winapi64::kernel32::LoadLibraryA,
        winapi64_get_proc_address: mwemu_winapi64::kernel32::GetProcAddress,
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
