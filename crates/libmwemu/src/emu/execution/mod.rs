use std::io::Write as _;
use std::sync::atomic;

use iced_x86::Instruction;

use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::disassemble::InstructionCache;
use crate::err::MwemuError;
use crate::serialization;
use crate::windows::peb::peb64;
// Architecture precondition guards
// ---------------------------------------------------------------------------
//
// The execution layer exposes two parallel public APIs:
//
//   * Generic compatibility wrappers (`run`, `step`, `decode_and_execute`,
//     `advance_pc`, `run_single_threaded`, `run_multi_threaded`) that
//     dispatch to the right ISA based on `cfg.arch`.
//
//   * ISA-specific public APIs (`run_x86`, `run_aarch64`, `step_x86`,
//     `step_aarch64`, `decode_and_execute_x86`, `decode_and_execute_aarch64`,
//     `advance_pc_x86`, `advance_pc_aarch64`, `run_single_threaded_x86`,
//     `run_single_threaded_aarch64`, `run_multi_threaded_x86`,
//     `run_multi_threaded_aarch64`) whose name guarantees the selected ISA.
//
// The ISA-specific methods validate their precondition via the helpers below
// and panic with a consistent message identifying the offending method and the
// configured architecture. These assertions exist so callers cannot accidentally
// drive an AArch64 program through x86-only register/cache accessors (or vice
// versa); the existing `ArchState` accessors remain the final invariant check
// inside the typed code paths.

#[inline]
pub(crate) fn assert_x86_arch(emu: &Emu, method: &'static str) {
    if !emu.cfg.arch.is_x86() {
        panic!(
            "{} called on non-x86 emulator (arch={:?}); use the AArch64 API instead",
            method, emu.cfg.arch
        );
    }
}

#[inline]
pub(crate) fn assert_aarch64_arch(emu: &Emu, method: &'static str) {
    if !emu.cfg.arch.is_aarch64() {
        panic!(
            "{} called on non-AArch64 emulator (arch={:?}); use the x86 API instead",
            method, emu.cfg.arch
        );
    }
}

mod aarch64;
mod control;
mod entropy;
mod x86;
// Re-export the ISA-neutral types that the ISA submodules use through
// `use super::{ArchState, Emu};`. Keeping the re-exports here means the
// submodules don't need a sibling crate path or an `extern crate` shortcut.
pub(crate) use crate::emu::Emu;

impl Emu {
    #[inline]
    pub fn stop(&mut self) {
        self.process_terminated = true;
        self.is_running.store(0, atomic::Ordering::Relaxed);
    }

    #[inline]
    fn needs_trace_file_instruction(&self) -> bool {
        self.cfg.trace_regs && self.cfg.trace_filename.is_some() && self.pos >= self.cfg.trace_start
    }

    #[inline]
    pub(crate) fn needs_decoded_instruction_for_observers(&self) -> bool {
        self.hooks.hook_on_pre_instruction.is_some()
            || self.hooks.hook_on_post_instruction.is_some()
            || self.needs_trace_file_instruction()
            || self.cfg.verbose >= 2
    }

    #[inline]
    pub(crate) fn clear_last_decoded_instruction(&mut self) {
        self.last_decoded = None;
        self.last_decoded_addr = 0;
    }

    #[inline]
    pub(crate) fn last_decoded_x86(&mut self, addr: u64, ins: Instruction) -> DecodedInstruction {
        let decoded = DecodedInstruction::X86(ins);
        self.last_decoded = Some(decoded);
        self.last_decoded_addr = addr;
        decoded
    }

    #[inline]
    pub(crate) fn last_decoded_aarch64(
        &mut self,
        addr: u64,
        ins: yaxpeax_arm::armv8::a64::Instruction,
    ) -> DecodedInstruction {
        let decoded = DecodedInstruction::AArch64(ins);
        self.last_decoded = Some(decoded);
        self.last_decoded_addr = addr;
        decoded
    }

    /// Decode and execute one instruction at the current PC.
    /// Returns (instruction_size, emulation_ok).
    /// Dispatches to x86 or aarch64 decode/execute internally.
    #[inline]
    pub fn decode_and_execute(&mut self) -> (usize, bool) {
        if self.cfg.arch.is_aarch64() {
            self.decode_and_execute_aarch64()
        } else {
            self.decode_and_execute_x86()
        }
    }

    /// Advance the program counter by `sz` bytes.
    /// Respects force_reload (branch already set PC).
    /// Dispatches to the ISA-specific `advance_pc_*` helper.
    #[inline]
    pub fn advance_pc(&mut self, sz: usize) {
        if self.cfg.arch.is_aarch64() {
            self.advance_pc_aarch64(sz);
        } else {
            self.advance_pc_x86(sz);
        }
    }

    /// Emulate a single step from the current point.
    /// Works for both x86 and aarch64. Handles hooks, threading, exit_position.
    #[inline]
    pub fn step(&mut self) -> bool {
        if self.cfg.arch.is_aarch64() {
            self.step_aarch64()
        } else {
            self.step_x86()
        }
    }

    /// Generic single-thread run dispatcher. The ISA-specific loops live in
    /// `run_single_threaded_x86` / `run_single_threaded_aarch64`; this
    /// wrapper just picks one based on the configured architecture.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading and ISA selection"
    )]
    pub fn run_single_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        if self.cfg.arch.is_aarch64() {
            self.run_single_threaded_aarch64(end_addr)
        } else {
            self.run_single_threaded_x86(end_addr)
        }
    }

    /// Generic multi-thread run dispatcher. The ISA-specific loops live in
    /// `run_multi_threaded_x86` / `run_multi_threaded_aarch64`; this
    /// wrapper just picks one based on the configured architecture.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading and ISA selection"
    )]
    pub fn run_multi_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        if self.cfg.arch.is_aarch64() {
            self.run_multi_threaded_aarch64(end_addr)
        } else {
            self.run_multi_threaded_x86(end_addr)
        }
    }

    /// Start emulation until a ret instruction is found.
    /// It will return the address or MwemuError.
    #[inline]
    pub fn run_until_ret(&mut self) -> Result<u64, MwemuError> {
        self.run_until_ret = true;
        let result = self.run(None);
        self.run_until_ret = false;
        result
    }

    /// Generic multi-thread step dispatcher used by the threaded scheduler.
    /// The ISA-specific loops live in
    /// `run_multi_threaded_x86` / `run_multi_threaded_aarch64`; this wrapper
    /// delegates via the scheduler which itself calls the arch-dispatched
    /// `decode_and_execute` / `advance_pc` on `Emu`.
    #[allow(deprecated)]
    pub fn step_multi_threaded(&mut self) -> bool {
        self.pos += 1;

        // exit
        if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
            log::trace!("exit position reached");

            if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(self, self.cfg.dump_filename.as_ref().unwrap());
            }

            if self.cfg.trace_regs && self.cfg.trace_filename.is_some() {
                self.trace_file
                    .as_ref()
                    .unwrap()
                    .flush()
                    .expect("failed to flush trace file");
            }

            return false;
        }

        // Thread scheduling - find next runnable thread
        let num_threads = self.threads.len();
        let current_tick = self.tick;

        let current_can_run = !self.threads[self.current_thread_id].suspended
            && self.threads[self.current_thread_id].wake_tick <= current_tick
            && self.threads[self.current_thread_id].blocked_on_cs.is_none();

        if num_threads > 1 {
            for i in 0..num_threads {
                let thread_idx = (self.current_thread_id + i + 1) % num_threads;
                let thread = &self.threads[thread_idx];

                if !thread.suspended
                    && thread.wake_tick <= current_tick
                    && thread.blocked_on_cs.is_none()
                {
                    return crate::threading::scheduler::ThreadScheduler::execute_thread_instruction(
                        self, thread_idx,
                    );
                }
            }

            log::debug!("No other threads runnable, checking current thread");
        }

        if current_can_run {
            return crate::threading::scheduler::ThreadScheduler::execute_thread_instruction(
                self,
                self.current_thread_id,
            );
        }

        // All threads blocked - advance time to next wake point.
        let mut next_wake = usize::MAX;
        for thread in &self.threads {
            if !thread.suspended && thread.wake_tick > current_tick {
                next_wake = next_wake.min(thread.wake_tick);
            }
        }

        if next_wake != usize::MAX && next_wake > current_tick {
            self.tick = next_wake;
            log::trace!(
                "⏰ All threads blocked, advancing tick from {} to {}",
                current_tick,
                next_wake
            );
            return self.step();
        }

        log::trace!("💀 All threads are blocked/suspended, cannot continue execution");
        if num_threads > 1 {
            log::trace!("Final thread states:");
            for (i, thread) in self.threads.iter().enumerate() {
                log::trace!(
                    "  Thread[{}]: ID=0x{:x}, suspended={}, wake_tick={}, blocked={}",
                    i,
                    thread.id,
                    thread.suspended,
                    thread.wake_tick,
                    thread.blocked_on_cs.is_some()
                );
            }
        }
        false
    }

    /// Run until a specific position (emu.pos)
    /// This don't reset the emu.pos, will meulate from current position to
    /// selected end_pos included.
    pub fn run_to(&mut self, end_pos: u64) -> Result<u64, MwemuError> {
        self.max_pos = Some(end_pos);
        let r = self.run(None);
        self.max_pos = None;
        r
    }

    /// Start or continue emulation.
    /// For emulating forever: run(None)
    /// For emulating until an address: run(Some(0x11223344))
    /// self.pos is not set to zero, can be used to continue emulation.
    /// Automatically dispatches to single or multi-threaded execution based on cfg.enable_threading.
    #[inline]
    pub fn run(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        if self.cfg.arch.is_aarch64() {
            self.run_aarch64(end_addr)
        } else {
            self.run_x86(end_addr)
        }
    }

    /// Reset the instruction cache for the currently-configured architecture.
    /// Used by the ISA-specific `run_*` entry points.
    pub(crate) fn reset_active_instruction_cache(&mut self) {
        self.instruction_state.instruction_cache = InstructionCache::default();
    }

    /// Shared PEB setup that runs before dispatching to the cached loop, for
    /// both x86 and AArch64 entry points. Caller must have already validated
    /// the architecture.
    pub(crate) fn run_preflight(&mut self) -> Result<(), MwemuError> {
        if !self.os.is_linux()
            && self.cfg.arch.is_64bits()
            && self.cfg.ssdt_use_ldr_initialize_thunk
            && self.maps.get_map_by_name("peb").is_some()
        {
            peb64::ensure_peb_system_dependent_07(self);
        }
        Ok(())
    }

    /// Shared ISA-agnostic step body: preflight, threading dispatch, exit
    /// position, decode-and-execute, and PC advance. The decode/advance calls
    /// already dispatch through `decode_and_execute` / `advance_pc` which
    /// respect the configured architecture at runtime.
    #[allow(deprecated)]
    pub(crate) fn step_isa(&mut self) -> bool {
        if self.process_terminated {
            return false;
        }

        if !self.os.is_linux()
            && self.cfg.arch.is_64bits()
            && self.cfg.ssdt_use_ldr_initialize_thunk
        {
            peb64::ensure_peb_system_dependent_07(self);
        }

        // Multi-threaded dispatch (uses scheduler which calls decode_and_execute internally)
        if self.cfg.enable_threading && self.threads.len() > 1 {
            return self.step_multi_threaded();
        }

        self.pos += 1;

        // exit position check
        if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
            log::trace!("exit position reached");
            if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(self, self.cfg.dump_filename.as_ref().unwrap());
            }
            if self.cfg.trace_regs && self.cfg.trace_filename.is_some() {
                self.trace_file
                    .as_ref()
                    .unwrap()
                    .flush()
                    .expect("failed to flush trace file");
            }
            return false;
        }

        // Decode and execute (arch-dispatched)
        let (sz, result_ok) = self.decode_and_execute();
        if sz == 0 {
            return false;
        }

        // Advance PC
        self.advance_pc(sz);

        result_ok
    }

    /// Returns true when the active instruction cache (per `cfg.arch`) can
    /// still emit another instruction from its decoded block. Used by both
    /// ISA-specific single-thread loops.
    pub(crate) fn instruction_cache_can_decode(&self) -> bool {
        self.instruction_state.instruction_cache.can_decode()
    }
}
