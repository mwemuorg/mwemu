#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::assertions_on_constants)]
// --- clippy v1 burn-down backlog (see V1-ROADMAP.md P2 #9) ---
// These lints are grandfathered so CI can run `-D warnings` and fail on any
// NEW warning outside this set. Remove entries as the backlog is cleared.
// NOTE: `neg_cmp_op_on_partial_ord` is kept deliberately — in the float
// compare handlers (cmpss/cmpsd/cmpps/cmppd/avx) `!(a < b)` is NOT the same
// as `a >= b` for NaN operands, which is the intended x86 semantics.
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::duplicate_mod)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::explicit_counter_loop)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::legacy_numeric_constants)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::lines_filter_map_ok)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::single_match)]
#![allow(clippy::suspicious_open_options)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::while_let_loop)]

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

pub fn emu64() -> Emu {
    let mut emu = Emu::new(Arch::X86_64);
    emu.disable_ctrlc();
    emu
}

pub fn emu32() -> Emu {
    let mut emu = Emu::new(Arch::X86);
    emu.disable_ctrlc();
    emu
}

pub fn emu_aarch64() -> Emu {
    let mut emu = Emu::new(Arch::Aarch64);
    emu.disable_ctrlc();
    emu
}
