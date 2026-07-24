//! x86-family execution implementations.
//!
//! Modules under this folder only run when `cfg.arch.is_x86()` is true and
//! provide the typed x86 entry points (`run_x86`, `step_x86`,
//! `decode_and_execute_x86`, `advance_pc_x86`, `run_single_threaded_x86`,
//! `run_multi_threaded_x86`), the x86 calling-convention helpers, the x86 REP
//! fast-path and state machine, the x86 SSDT API-shim cache, the x86 cache
//! fill, and the x86 cached run loops. The public generic facade in
//! `super::mod` dispatches into this folder for x86 emulators.

mod call;
mod decode;
mod multithreaded;
mod rep;
mod shim;
mod single_threaded;

use crate::err::MwemuError;
// Re-export the ISA-neutral types that the inner x86 modules use through
// `use super::{ArchState, Emu};`. Both come from `crate::emu`.
pub(crate) use crate::emu::{ArchState, Emu};
// Re-export the ISA guard so the inner x86 files can call it via
// `assert_x86_arch(self, ...)` (the guard is a free function in the parent
// module, not an associated method).
pub(crate) use super::assert_x86_arch;

impl Emu {
    /// x86-family variant of `step`. Panics if the configured architecture is
    /// AArch64.
    pub fn step_x86(&mut self) -> bool {
        assert_x86_arch(self, "step_x86");
        self.step_isa()
    }

    /// x86-family entry point. Performs the same preflight as `run`, resets
    /// the x86 instruction cache, and dispatches to single- or multi-threaded
    /// execution. Panics if the configured architecture is AArch64.
    pub fn run_x86(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        assert_x86_arch(self, "run_x86");
        self.reset_active_instruction_cache();
        self.run_preflight()?;
        if self.cfg.enable_threading && self.threads.len() > 1 {
            self.run_multi_threaded_x86(end_addr)
        } else {
            self.run_single_threaded_x86(end_addr)
        }
    }

    /// x86-family variant of `decode_and_execute`. Panics if the configured
    /// architecture is AArch64.
    pub fn decode_and_execute_x86(&mut self) -> (usize, bool) {
        self::decode::decode_and_execute_x86(self)
    }

    /// x86-family variant of `advance_pc`. Respects `force_reload` and then
    /// advances RIP (64-bit) or EIP (32-bit) by `sz` bytes. Panics on AArch64.
    #[inline]
    pub fn advance_pc_x86(&mut self, sz: usize) {
        self::decode::advance_pc_x86(self, sz)
    }
}
