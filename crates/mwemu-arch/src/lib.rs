//! CPU state of mwemu: the x86 and AArch64 registers, the flags and the FPU.
//!
//! This crate knows nothing about the emulator: `mwemu-core` keeps this state
//! in `Emu` and re-exports the crate as `mwemu_core::arch`. `libmwemu`
//! re-exports it as `libmwemu::arch`.
//!
//! The module keeps the path it had inside `mwemu-core` (`arch`), so the
//! files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
// The `construct_uint!` macro from `uint` generates code with manual div_ceil;
// we cannot fix it inside the macro expansion.
#![allow(clippy::manual_div_ceil)]

#[macro_use]
mod macros;

pub mod arch;

/// Helpers the register code needs. `mwemu-core` re-exports them as
/// `mwemu_core::utils::bits`.
pub mod utils {
    pub mod bits;
}

// The code names these as it did when it was part of mwemu-core. Keep those
// paths valid inside this crate.
pub(crate) use arch::x86::fpu;
pub(crate) use arch::x86::regs as regs64;
pub(crate) use mwemu_maps::maps;
