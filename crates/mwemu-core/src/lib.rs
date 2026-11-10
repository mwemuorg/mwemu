//! The mwemu emulator core: CPU engines, memory maps, the run loop and the
//! loaders (PE, ELF, Mach-O).
//!
//! This is an internal crate. Use `libmwemu`, which re-exports the supported
//! API and adds the layers built on the core: the OS layers (`mwemu-linux`,
//! `mwemu-macos`, `mwemu-windows`), the debugger (`mwemu-debug`) and the
//! Windows API handlers (`mwemu-winapi32`, `mwemu-winapi64`). The core reaches
//! them through [`layers`] and [`api::dispatch`]. Everything here is `pub` so
//! those crates can reach it; none of it is a stability promise.

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

pub mod api;
pub mod arch;
pub mod config;
pub mod debug;
pub mod emu;
pub mod engine;
pub mod err;
pub mod exception;
pub mod hooks;
pub mod kernel;
pub mod layers;
pub mod maps;
pub mod threading;
pub mod windows;

// Short names the code base uses for these modules.
pub use arch::aarch64::regs as regs_aarch64;
pub use arch::x86::context;
pub use arch::x86::eflags;
pub use arch::x86::flags;
pub use arch::x86::fpu;
pub use arch::x86::regs as regs64;

pub use api::windows as winapi;

pub use debug::console;
pub use emu::emu_context;
pub use utils::color_enabled;
pub use utils::disable_color;

/// `libmwemu::emu32()` for this crate's own unit tests.
#[cfg(test)]
fn emu32() -> emu::Emu {
    let mut emu = emu::Emu::new(arch::Arch::X86);
    emu.disable_ctrlc();
    emu
}

/// `libmwemu::emu64()` for this crate's own unit tests.
#[cfg(test)]
fn emu64() -> emu::Emu {
    let mut emu = emu::Emu::new(arch::Arch::X86_64);
    emu.disable_ctrlc();
    emu
}

/// Shared helpers for this crate's own unit tests (`libmwemu` has the same
/// ones in its `tests` module).
#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::OnceLock;
    use tempfile::TempDir;

    /// Keep the global file-system root alive while independent tests use it.
    pub(crate) fn file_system_root() -> &'static Path {
        static ROOT: OnceLock<TempDir> = OnceLock::new();
        ROOT.get_or_init(|| {
            let root = tempfile::tempdir().unwrap();
            crate::emu::object_handle::file_handle::init_file_system(Some(root.path())).unwrap();
            root
        })
        .path()
    }
}
