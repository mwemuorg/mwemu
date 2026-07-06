use crate::emu::Emu;
use log;
use std::io::Write;

/// Thread scheduling and management functions for the emulator
pub struct ThreadScheduler;

impl ThreadScheduler {
    /// Schedule and switch to the next runnable thread (round-robin)
    /// Always tries to switch to give each thread equal time
    /// Returns true if a thread switch occurred, false otherwise
    pub fn schedule_next_thread(emu: &mut Emu) -> bool {
        // Single thread case - no scheduling needed
        if emu.threads.len() <= 1 {
            return false;
        }

        let current_tick = emu.tick;
        let current_thread_id = emu.current_thread_id;

        // Round-robin: always try to switch to the next thread
        // This ensures fair scheduling - each thread gets one instruction
        for i in 1..=emu.threads.len() {
            let thread_idx = (current_thread_id + i) % emu.threads.len();

            // Skip back to current thread only if no other threads are runnable
            if thread_idx == current_thread_id {
                // We've checked all other threads, none are runnable
                // Check if current thread can continue
                if Self::is_thread_runnable(emu, current_thread_id) {
                    return false; // Stay on current thread
                }
                // Current thread also can't run
                break;
            }

            if Self::is_thread_runnable(emu, thread_idx) {
                // Found a runnable thread - switch to it
                /*log::debug!(
                    "🔄 Thread switch: {} -> {} at step {}",
                    current_thread_id,
                    thread_idx,
                    emu.pos
                );
                log::debug!(
                    "   From RIP: 0x{:x} -> To RIP: 0x{:x}",
                    emu.threads[current_thread_id].regs_x86().rip,
                    emu.threads[thread_idx].regs_x86().rip
                );*/

                Self::switch_to_thread(emu, thread_idx);
                return true;
            }
        }

        // No threads are runnable (including current)
        // Try to advance time if threads are just sleeping
        if Self::advance_to_next_wake(emu) {
            // Recursively try scheduling again after time advance
            return Self::schedule_next_thread(emu);
        }

        // All threads are permanently blocked
        Self::log_thread_states(emu);
        log::error!("⚠️ All threads blocked or suspended - deadlock detected");

        false
    }

    /// Check if a specific thread is runnable
    fn is_thread_runnable(emu: &Emu, thread_idx: usize) -> bool {
        if thread_idx >= emu.threads.len() {
            return false;
        }

        let thread = &emu.threads[thread_idx];
        !thread.suspended && thread.wake_tick <= emu.tick && thread.blocked_on_cs.is_none()
    }

    /// Advance emulator tick to the next thread wake time
    /// Returns true if time was advanced, false if no threads are waiting
    fn advance_to_next_wake(emu: &mut Emu) -> bool {
        let current_tick = emu.tick;
        let mut next_wake = usize::MAX;

        // Find the earliest wake time among suspended threads
        for thread in &emu.threads {
            if !thread.suspended && thread.wake_tick > current_tick {
                next_wake = next_wake.min(thread.wake_tick);
            }
        }

        if next_wake != usize::MAX && next_wake > current_tick {
            log::trace!(
                "⏰ Advancing tick from {} to {} (all threads sleeping)",
                current_tick,
                next_wake
            );
            emu.tick = next_wake;
            return true;
        }

        false
    }

    /// Log the current state of all threads for debugging
    pub fn log_thread_states(emu: &Emu) {
        log::trace!("=== Thread States ===");
        for (i, thread) in emu.threads.iter().enumerate() {
            let status = Self::get_thread_status_string(emu, i);
            let marker = if i == emu.current_thread_id {
                ">>>"
            } else {
                "   "
            };

            let thread_pc = match &thread.arch {
                crate::threading::context::ArchThreadState::X86 { regs, .. } => regs.rip,
                crate::threading::context::ArchThreadState::AArch64 { regs, .. } => regs.pc,
            };
            log::trace!(
                "{} Thread[{}]: ID=0x{:x}, PC=0x{:x}, Status={}",
                marker,
                i,
                thread.id,
                thread_pc,
                status
            );
        }
        log::trace!("Current tick: {}", emu.tick);
    }

    /// Get a human-readable status string for a thread
    fn get_thread_status_string(emu: &Emu, thread_idx: usize) -> String {
        let thread = &emu.threads[thread_idx];

        if thread.suspended {
            "SUSPENDED".to_string()
        } else if thread.wake_tick > emu.tick {
            format!("SLEEPING(wake={})", thread.wake_tick)
        } else if thread.blocked_on_cs.is_some() {
            "BLOCKED_CS".to_string()
        } else {
            "RUNNABLE".to_string()
        }
    }

    /// Switch execution context to a different thread
    pub fn switch_to_thread(emu: &mut Emu, thread_id: usize) -> bool {
        if thread_id >= emu.threads.len() {
            log::error!("Invalid thread ID: {}", thread_id);
            return false;
        }

        if thread_id == emu.current_thread_id {
            return true; // Already on this thread
        }

        // Switch to new thread (FPU state is inside the ArchThreadState enum,
        // so it moves with the thread automatically)
        emu.current_thread_id = thread_id;

        // Don't set force_reload - we want the thread to continue from its current position
        // force_reload would prevent IP advancement which causes instructions to execute twice

        /*log::trace!(
            "Switched to thread {} (ID: 0x{:x})",
            thread_id,
            emu.threads[thread_id].id
        );*/

        true
    }

    /// Execute a single instruction for a specific thread.
    /// Uses the arch-dispatched decode_and_execute() and advance_pc() on Emu.
    pub fn execute_thread_instruction(emu: &mut Emu, thread_id: usize) -> bool {
        // Switch to target thread if needed
        if emu.current_thread_id != thread_id
            && !Self::switch_to_thread(emu, thread_id) {
                return false;
            }

        let pc = emu.pc();

        // Decode and execute
        let (sz, result_ok) = emu.decode_and_execute();
        if sz == 0 {
            return false;
        }

        // Post instruction hook (fires for both arches via DecodedInstruction)
        if let Some(mut hook_fn) = emu.hooks.hook_on_post_instruction.take() {
            let decoded = emu.last_decoded.unwrap();
            hook_fn(emu, pc, &decoded, sz, result_ok);
            emu.hooks.hook_on_post_instruction = Some(hook_fn);
        }

        // Advance instruction pointer
        emu.advance_pc(sz);

        result_ok
    }

    /// Main thread scheduling step - replaces the complex logic in step()
    pub fn step_with_scheduling(emu: &mut Emu) -> bool {
        emu.pos += 1;

        // Check exit condition
        if emu.cfg.exit_position != 0 && emu.pos == emu.cfg.exit_position {
            log::trace!("Exit position reached");
            Self::handle_exit(emu);
            return false;
        }

        // If only one thread, execute it directly
        if emu.threads.len() == 1 {
            if Self::is_thread_runnable(emu, 0) {
                return Self::execute_thread_instruction(emu, 0);
            } else {
                log::error!("Single thread is not runnable");
                return false;
            }
        }

        // Multi-threaded execution with scheduling

        // First, try to continue with current thread if it's still runnable
        if Self::is_thread_runnable(emu, emu.current_thread_id) {
            // Give current thread another timeslice
            return Self::execute_thread_instruction(emu, emu.current_thread_id);
        }

        // Current thread can't run, find another
        for i in 1..emu.threads.len() {
            let thread_idx = (emu.current_thread_id + i) % emu.threads.len();
            if Self::is_thread_runnable(emu, thread_idx) {
                log::debug!(
                    "Switching from thread {} to {}",
                    emu.current_thread_id,
                    thread_idx
                );
                return Self::execute_thread_instruction(emu, thread_idx);
            }
        }

        // No threads are immediately runnable - try advancing time
        if Self::advance_to_next_wake(emu) {
            // Time advanced, try again
            return Self::step_with_scheduling(emu);
        }

        // All threads are blocked
        Self::log_thread_states(emu);
        log::error!("All threads are blocked or suspended");
        false
    }

    /// Handle emulator exit
    fn handle_exit(emu: &mut Emu) {
        if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
            crate::serialization::Serialization::dump(emu, emu.cfg.dump_filename.as_ref().unwrap());
        }

        if emu.cfg.trace_regs && emu.cfg.trace_filename.is_some() {
            emu.trace_file
                .as_ref()
                .unwrap()
                .flush()
                .expect("failed to flush trace file");
        }
    }
}
