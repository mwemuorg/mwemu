//! libmwemu: an x86, x86-64 and AArch64 emulator for malware analysis.
//!
//! This crate is the public face of three internal ones: the emulator core
//! (`mwemu-core`) and the Windows API handlers (`mwemu-winapi32`,
//! `mwemu-winapi64`). It re-exports the core's API and plugs the handlers in.

// Lints the test suite was written under.
#![cfg_attr(
    test,
    allow(non_snake_case, dead_code, unused_variables, unused_must_use)
)]
#![cfg_attr(test, allow(clippy::assertions_on_constants))]

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
pub(crate) use mwemu_core::{engine, exception, threading, windows};
#[cfg(test)]
pub(crate) mod syscall {
    pub(crate) use mwemu_core::syscall::*;
    pub(crate) use mwemu_linux::syscall::linux;
}
#[cfg(test)]
pub(crate) mod api {
    pub(crate) use mwemu_core::api::*;
    pub(crate) use mwemu_linux::api::linux;
    pub(crate) use mwemu_macos::api::macos;
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

/// Register the Windows API handlers and the OS layers (`mwemu-linux`,
/// `mwemu-macos`) with the emulator core. `emu32()`, `emu64()` and
/// `emu_aarch64()` do it for you; call it yourself before running an emulator
/// built another way (`Emu::new`, `Serialization::load`). Calling it more than
/// once is harmless.
pub fn register_api_handlers() {
    mwemu_linux::register();
    mwemu_macos::register();
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
