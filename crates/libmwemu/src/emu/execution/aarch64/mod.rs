//! AArch64-family execution implementations.
//!
//! Modules under this folder only run when `cfg.arch.is_aarch64()` is true
//! and provide the typed AArch64 entry points (`run_aarch64`, `step_aarch64`,
//! `decode_and_execute_aarch64`, `advance_pc_aarch64`,
//! `run_single_threaded_aarch64`, `run_multi_threaded_aarch64`), the
//! AArch64 AAPCS64 calling-convention helper, the AArch64 cache fill, and the
//! AArch64 cached run loops. The public generic facade in `super::mod`
//! dispatches into this folder for AArch64 emulators.

mod call;
mod decode;
mod multithreaded;
mod single_threaded;

use crate::err::MwemuError;
// Re-export the ISA-neutral types that the inner aarch64 modules use through
// `use super::{ArchState, Emu};`. Both come from `crate::emu`.
pub(crate) use crate::emu::Emu;
// Re-export the ISA guard so the inner aarch64 files can call it via
// `assert_aarch64_arch(self, ...)` (the guard is a free function in the
// parent module, not an associated method).
pub(crate) use super::assert_aarch64_arch;

impl Emu {
    /// AArch64 variant of `step`. Panics if the configured architecture is
    /// not AArch64.
    pub fn step_aarch64(&mut self) -> bool {
        assert_aarch64_arch(self, "step_aarch64");
        self.step_isa()
    }

    /// AArch64 entry point. Performs the same preflight as `run`, resets the
    /// AArch64 instruction cache, and dispatches to single- or multi-threaded
    /// execution. Panics if the configured architecture is not AArch64.
    pub fn run_aarch64(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        assert_aarch64_arch(self, "run_aarch64");
        self.reset_active_instruction_cache();
        self.run_preflight()?;
        if self.cfg.enable_threading && self.threads.len() > 1 {
            self.run_multi_threaded_aarch64(end_addr)
        } else {
            self.run_single_threaded_aarch64(end_addr)
        }
    }

    /// AArch64 variant of `decode_and_execute`. Panics if the configured
    /// architecture is not AArch64.
    pub fn decode_and_execute_aarch64(&mut self) -> (usize, bool) {
        self::decode::decode_and_execute_aarch64(self)
    }

    /// AArch64 variant of `advance_pc`. Respects `force_reload` and otherwise
    /// advances PC by `sz` bytes (normal decoders always pass 4). Panics on x86.
    #[inline]
    pub fn advance_pc_aarch64(&mut self, sz: usize) {
        self::decode::advance_pc_aarch64(self, sz)
    }
}
