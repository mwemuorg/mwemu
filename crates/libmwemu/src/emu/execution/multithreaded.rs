use std::io::Write as _;
use std::sync::atomic;

use crate::debug::console::Console;
use crate::err::MwemuError;
use crate::serialization;

use super::Emu;

impl Emu {
    /// Start or continue emulation (multi-threaded implementation).
    /// For emulating forever: run(None)
    /// For emulating until an address: run(Some(0x11223344))
    /// self.pos is not set to zero, can be used to continue emulation.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading"
    )]
    #[allow(deprecated)] // delegates to step_multi_threaded (also deprecated)
    pub fn run_multi_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        if self.process_terminated {
            return Err(MwemuError::new("process terminated (NtTerminateProcess)"));
        }

        self.ensure_run_start_pc_mapped(self.regs().rip)?;

        self.is_running.store(1, atomic::Ordering::Relaxed);
        self.install_ctrlc_handler_if_enabled();

        let mut looped: Vec<u64> = Vec::new();
        let mut prev_addr: u64 = 0;
        let mut repeat_counter: u32 = 0;

        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                let rip = self.regs().rip;

                if self.maps.get_mem_by_addr(rip).is_none() {
                    log::trace!("redirecting code flow to non mapped address 0x{:x}", rip);
                    Console::spawn_console(self);
                    return Err(MwemuError::new("cannot read program counter"));
                }

                if let Some(pc) = self.reached_outer_run_limit(rip, end_addr) {
                    return Ok(pc);
                }

                let next_pos = self.pos.saturating_add(1);

                if (self.exp != u64::MAX && self.exp == next_pos)
                    || self.bp.is_bp_instruction(next_pos)
                    || self.bp.is_bp(rip)
                    || (self.cfg.console2 && self.cfg.console_addr == rip)
                {
                    if self.running_script {
                        return Ok(rip);
                    }
                    self.cfg.console2 = false;
                    if self.cfg.verbose >= 2 {
                        log::trace!(
                            "------- (breakpoint/console at 0x{:x}, pos {})",
                            rip,
                            next_pos
                        );
                    }
                    Console::spawn_console(self);
                    if self.force_break {
                        self.force_break = false;
                        break;
                    }
                    continue;
                }

                self.observe_loop_progress(
                    rip,
                    &mut prev_addr,
                    &mut repeat_counter,
                    &mut looped,
                    "inifinite loop found",
                )?;

                self.trace_pre_step_state(next_pos);

                let step_ok = self.step_multi_threaded();

                self.instruction_count = self.instruction_count.saturating_add(1);

                if let Some(pc) = self.check_runtime_limits(self.regs().rip) {
                    return Ok(pc);
                }

                self.update_verbose_at();
                self.update_verbose_range();

                if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                    return Ok(self.regs().rip);
                }

                if self.cfg.entropy && self.instruction_count % 10000 == 0 {
                    self.update_entropy();
                }

                if self.cfg.trace_regs
                    && self.cfg.trace_filename.is_some()
                    && self.pos >= self.cfg.trace_start
                    && self.x86_instruction().is_some()
                {
                    self.capture_post_op();
                    self.write_to_trace_file();
                }

                if self.cfg.inspect {
                    self.trace_memory_inspection();
                }

                if !step_ok {
                    if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
                        return Ok(self.regs().rip);
                    }
                    let any_runnable = self.threads.iter().any(|t| {
                        !t.suspended && t.wake_tick <= self.tick && t.blocked_on_cs.is_none()
                    });
                    if !any_runnable {
                        return Err(MwemuError::new("all emulated threads blocked or suspended"));
                    }
                    if self.cfg.console_enabled {
                        Console::spawn_console(self);
                    } else if self.running_script {
                        return Ok(self.regs().rip);
                    } else {
                        return Err(MwemuError::new(&format!(
                            "emulation error at pos = {} rip = 0x{:x}",
                            self.pos,
                            self.regs().rip
                        )));
                    }
                }

                if self.force_break {
                    self.force_break = false;
                    break;
                }

                if self.is_api_run && self.is_break_on_api {
                    self.is_api_run = false;
                    break;
                }
            }

            if self.is_break_on_api {
                return Ok(0);
            }

            self.is_running.store(1, atomic::Ordering::Relaxed);
            Console::spawn_console(self);
        }
    }

    /// Emulate a single step from the current point (multi-threaded implementation).
    /// this don't reset the emu.pos, that mark the number of emulated instructions and point to
    /// the current emulation moment.
    /// If you do a loop with emu.step() will have more control of the emulator but it will be
    /// slow.
    /// Is more convinient using run and run_to or even setting breakpoints.
    #[deprecated(
        since = "0.1.0",
        note = "Use step() instead, which automatically handles threading"
    )]
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

        // Debug logging for threading
        if num_threads > 1 {
            /*log::trace!("=== THREAD SCHEDULER DEBUG ===");
            log::trace!("Step {}: {} threads, current_thread_id={}, tick={}",
                    self.pos, num_threads, self.current_thread_id, current_tick);

            for (i, thread) in self.threads.iter().enumerate() {
                let status = if thread.suspended {
                    "SUSPENDED".to_string()
                } else if thread.wake_tick > current_tick {
                    format!("SLEEPING(wake={})", thread.wake_tick)
                } else if thread.blocked_on_cs.is_some() {
                    "BLOCKED_CS".to_string()
                } else {
                    "RUNNABLE".to_string()
                };

                let marker = if i == self.current_thread_id { ">>> " } else { "    " };
                log::trace!("{}Thread[{}]: ID=0x{:x}, RIP=0x{:x}, Status={}",
                        marker, i, thread.id, thread.regs_x86().rip, status);
            }*/
        }

        // Check if current thread can run
        let current_can_run = !self.threads[self.current_thread_id].suspended
            && self.threads[self.current_thread_id].wake_tick <= current_tick
            && self.threads[self.current_thread_id].blocked_on_cs.is_none();

        if num_threads > 1 {
            //log::debug!("Current thread {} can run: {}", self.current_thread_id, current_can_run);

            // Round-robin scheduling: try each thread starting from next one
            for i in 0..num_threads {
                let thread_idx = (self.current_thread_id + i + 1) % num_threads;
                let thread = &self.threads[thread_idx];

                /*log::debug!("Checking thread {}: suspended={}, wake_tick={}, blocked={}",
                thread_idx, thread.suspended, thread.wake_tick,
                thread.blocked_on_cs.is_some());*/

                // Check if thread is runnable
                if !thread.suspended
                    && thread.wake_tick <= current_tick
                    && thread.blocked_on_cs.is_none()
                {
                    // Found a runnable thread, execute it
                    if thread_idx != self.current_thread_id {
                        /*log::trace!("🔄 THREAD SWITCH: {} -> {} (step {})",
                                self.current_thread_id, thread_idx, self.pos);
                        log::trace!("   From RIP: 0x{:x} -> To RIP: 0x{:x}",
                                self.threads[self.current_thread_id].regs_x86().rip,
                                thread.regs_x86().rip);*/
                    }
                    return crate::threading::scheduler::ThreadScheduler::execute_thread_instruction(
                        self, thread_idx,
                    );
                }
            }

            log::debug!("No other threads runnable, checking current thread");
        }

        // If no other threads are runnable, try current thread
        if current_can_run {
            /*if num_threads > 1 {
                log::debug!("Continuing with current thread {}", self.current_thread_id);
            }*/
            return crate::threading::scheduler::ThreadScheduler::execute_thread_instruction(
                self,
                self.current_thread_id,
            );
        }

        // All threads are blocked or suspended - advance time to next wake point
        let mut next_wake = usize::MAX;
        for thread in &self.threads {
            if !thread.suspended && thread.wake_tick > current_tick {
                next_wake = next_wake.min(thread.wake_tick);
            }
        }

        if next_wake != usize::MAX && next_wake > current_tick {
            // Advance time to next wake point
            self.tick = next_wake;
            log::trace!(
                "⏰ All threads blocked, advancing tick from {} to {}",
                current_tick,
                next_wake
            );
            // Try scheduling again
            return self.step();
        }

        // All threads are permanently blocked or suspended
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
}
