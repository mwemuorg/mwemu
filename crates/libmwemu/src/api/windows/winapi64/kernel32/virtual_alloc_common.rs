//! Shared failure paths for the winapi64 kernel32 `Virtual*` allocators
//! (`VirtualAlloc`, `VirtualAllocEx`, `VirtualAllocExNuma`). Each allocator
//! builds a `ctx` string describing its arguments and routes invalid-parameter
//! / out-of-memory exits through these, instead of redefining them per file.
//!
//! Architecture-independent helpers (`permissions`, `round_up`) live in
//! [`crate::winapi::common::virtual_alloc`].

use crate::emu::Emu;
use crate::winapi::winapi64::kernel32::set_last_error;
use crate::windows::constants;

/// Log a parameter-validation failure, set `ERROR_INVALID_PARAMETER`, return 0.
pub(crate) fn fail(emu: &mut Emu, label: &str, ctx: &str, reason: &str) {
    log_red!(emu, "kernel32!{} {} = 0 reason: {}", label, ctx, reason);
    set_last_error(constants::ERROR_INVALID_PARAMETER);
    emu.regs_mut().rax = 0;
}

/// Log an out-of-memory failure, set `ERROR_NOT_ENOUGH_MEMORY`, return 0.
pub(crate) fn fail_oom(emu: &mut Emu, label: &str, ctx: &str) {
    log_red!(emu, "kernel32!{} {} = 0 reason: out of memory", label, ctx);
    set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY);
    emu.regs_mut().rax = 0;
}
