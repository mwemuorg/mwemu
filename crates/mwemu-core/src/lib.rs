//! The mwemu emulator core: `Emu`, its methods and the run loop.
//!
//! This is an internal crate. Use `libmwemu`, which re-exports the supported
//! API and adds the crates built on the core: the CPU engines (`mwemu-x86`,
//! `mwemu-aarch64`), the OS layers (`mwemu-linux`, `mwemu-macos`,
//! `mwemu-windows`), the kernel API surfaces (`mwemu-kernel`), the debugger
//! (`mwemu-debug`) and the Windows API handlers (`mwemu-winapi32`,
//! `mwemu-winapi64`). The core reaches them through [`layers`] and
//! [`api::dispatch`]. Loading a binary and setting the emulator up for its
//! operating system is in the OS layers too: `Emu::load_code()` finds out
//! what the file is and calls the layer for it.
//!
//! The data `Emu` holds that does not need `Emu` is in crates below this one,
//! re-exported here at the paths it always had: guest memory ([`maps`], from
//! `mwemu-maps`), the CPU state ([`arch`], from `mwemu-arch`), the Windows
//! structures and constants ([`windows::structures`], [`windows::constants`],
//! from `mwemu-wintypes`) and the handle tables ([`emu::object_handle`], from
//! `mwemu-handles`).
//!
//! Everything here is `pub` so those crates can reach it; none of it is a
//! stability promise.

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

// `get_bit!` and `set_bit!` are defined with the CPU state that uses them.
#[macro_use]
extern crate mwemu_arch;
pub use mwemu_arch::{get_bit, set_bit};

#[macro_use]
pub mod utils;

pub mod api;
pub mod config;
pub mod debug;
pub mod emu;
pub mod err;
pub mod exception;
pub mod hooks;
pub mod kernel;
pub mod layers;
pub mod threading;
pub mod windows;

// Guest memory and the CPU state are their own crates; they keep their
// paths here.
pub use mwemu_arch::arch;
pub use mwemu_maps::maps;

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
