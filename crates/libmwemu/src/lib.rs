//! libmwemu: an x86, x86-64 and AArch64 emulator for malware analysis.
//!
//! This crate is the public face of the internal ones: the emulator core
//! (`mwemu-core`), the OS layers (`mwemu-linux`, `mwemu-macos`,
//! `mwemu-windows`), the debugger (`mwemu-debug`) and the Windows API handlers
//! (`mwemu-winapi32`, `mwemu-winapi64`). It re-exports their API and plugs the
//! layers and the handlers into the core.

// Lints the test suite was written under.
#![cfg_attr(
    test,
    allow(non_snake_case, dead_code, unused_variables, unused_must_use)
)]
#![cfg_attr(test, allow(clippy::assertions_on_constants))]

pub use mwemu_core::{arch, config, emu, err, hooks, maps, utils};
// The core's kernel-mode machinery plus the per-OS API surfaces.
pub use mwemu_debug::serialization;
pub use mwemu_kernel::kernel;

/// Breakpoints, definitions and the console prompts are in the core; the
/// console's commands, the gdb server and the script runner are in
/// `mwemu-debug`.
pub mod debug {
    pub use mwemu_core::debug::{breakpoint, console, definitions};
    pub use mwemu_debug::debug::{commands, gdb, script, tracing};
}

// Backwards-compatible re-exports (arch)
pub use mwemu_core::{context, eflags, flags, fpu, regs_aarch64, regs64};

// Re-exports for external crates
pub use mwemu_core::{color_enabled, console, disable_color, emu_context};
pub use mwemu_debug::debug::{gdb, script};

// Macros
pub use mwemu_core::{color, get_bit, log_orange, log_red, popn, set_bit, to32};

// The tests were written inside the single-crate libmwemu and reach its
// internals as `crate::…`; keep those paths valid for them.
#[cfg(test)]
pub(crate) use mwemu_core::{exception, threading};
#[cfg(test)]
pub(crate) use mwemu_windows::windows;
#[cfg(test)]
pub(crate) use mwemu_x86::engine;
#[cfg(test)]
pub(crate) mod syscall {
    pub(crate) use mwemu_linux::syscall::linux;
    pub(crate) use mwemu_windows::syscall::windows;
}
#[cfg(test)]
pub(crate) mod api {
    pub(crate) use mwemu_core::api::*;
    pub(crate) use mwemu_linux::api::linux;
    pub(crate) use mwemu_macos::api::macos;
    pub(crate) mod windows {
        pub(crate) use mwemu_winapi32 as winapi32;
        pub(crate) use mwemu_winapi64 as winapi64;
        pub(crate) use mwemu_windows::api::windows::*;
    }
}
#[cfg(test)]
pub(crate) use api::windows as winapi;

#[cfg(test)]
mod tests;

use arch::Arch;
use emu::Emu;
use mwemu_core::api::dispatch::{self, ApiDispatch};

/// Register the Windows API handlers and the layers (`mwemu-linux`,
/// `mwemu-macos`, `mwemu-windows`, `mwemu-debug`) with the emulator core. `emu32()`, `emu64()` and
/// `emu_aarch64()` do it for you; call it yourself before running an emulator
/// built another way (`Emu::new`, `Serialization::load`). Calling it more than
/// once is harmless.
pub fn register_api_handlers() {
    mwemu_x86::register();
    mwemu_aarch64::register();
    mwemu_kernel::register();
    mwemu_debug::register();
    mwemu_linux::register();
    mwemu_macos::register();
    mwemu_windows::register();
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
