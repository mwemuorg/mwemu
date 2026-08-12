use std::sync::atomic;

use crate::debug::console::Console;
use crate::emu::Emu;
use crate::err::MwemuError;

impl Emu {
    /// AArch64 multi-threaded scheduler loop. Uses `pc()` (which reads the
    /// AArch64 PC) and `regs_aarch64()` rather than the x86 register accessors,
    /// fixing the prior failure that called `regs().rip` on AArch64 threads.
    /// Retains the same scheduler, breakpoint, limit, console, `run_until_ret`,
    /// and termination behavior as the x86 path. Panics if the configured
    /// architecture is not AArch64.
    #[allow(deprecated)]
    pub fn run_multi_threaded_aarch64(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        super::super::assert_aarch64_arch(self, "run_multi_threaded_aarch64");

        if self.process_terminated {
            return Err(MwemuError::new("process terminated (NtTerminateProcess)"));
        }

        self.ensure_run_start_pc_mapped(self.pc())?;

        self.is_running.store(1, atomic::Ordering::Relaxed);
        self.install_ctrlc_handler_if_enabled();

        let mut looped: Vec<u64> = Vec::new();
        let mut prev_addr: u64 = 0;
        let mut repeat_counter: u32 = 0;

        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                let pc = self.regs_aarch64().pc;

                if self.maps.get_mem_by_addr(pc).is_none() {
                    log::trace!("redirecting code flow to non mapped address 0x{:x}", pc);
                    Console::spawn_console(self);
                    return Err(MwemuError::new("cannot read program counter"));
                }

                if let Some(limit_pc) = self.reached_outer_run_limit(pc, end_addr) {
                    std::hint::cold_path();
                    return Ok(limit_pc);
                }

                let next_pos = self.pos.saturating_add(1);

                if (self.exp != u64::MAX && self.exp == next_pos)
                    || self.bp.is_bp_instruction(next_pos)
                    || self.bp.is_bp(pc)
                    || (self.cfg.console2 && self.cfg.console_addr == pc)
                {
                    if self.running_script {
                        return Ok(pc);
                    }
                    self.cfg.console2 = false;
                    if self.cfg.verbose >= 2 {
                        log::trace!(
                            "------- (breakpoint/console at 0x{:x}, pos {})",
                            pc,
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
                    pc,
                    &mut prev_addr,
                    &mut repeat_counter,
                    &mut looped,
                    "infinite loop found",
                )?;

                self.trace_pre_step_state(next_pos);

                let step_ok = self.step_multi_threaded();

                self.instruction_count = self.instruction_count.saturating_add(1);

                if let Some(limit_pc) = self.check_runtime_limits(self.regs_aarch64().pc) {
                    return Ok(limit_pc);
                }

                self.update_verbose_at();
                self.update_verbose_range();

                if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                    return Ok(self.regs_aarch64().pc);
                }

                if self.cfg.entropy && self.instruction_count % 10000 == 0 {
                    self.update_entropy();
                }

                if self.cfg.trace_regs
                    && self.cfg.trace_filename.is_some()
                    && self.pos >= self.cfg.trace_start
                {
                    self.capture_post_op();
                    self.write_to_trace_file();
                }

                if self.cfg.inspect {
                    self.trace_memory_inspection();
                }

                if !step_ok {
                    if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
                        return Ok(self.regs_aarch64().pc);
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
                        return Ok(self.regs_aarch64().pc);
                    } else {
                        return Err(MwemuError::new(&format!(
                            "emulation error at pos = {} pc = 0x{:x}",
                            self.pos,
                            self.regs_aarch64().pc
                        )));
                    }
                }

                if self.run_until_ret
                    && self
                        .last_decoded
                        .map(|decoded| decoded.is_return())
                        .unwrap_or(false)
                {
                    return Ok(self.pc());
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
}
