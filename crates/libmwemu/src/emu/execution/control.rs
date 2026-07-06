use std::sync::{Arc, atomic};

use crate::err::MwemuError;

use super::Emu;

impl Emu {
    pub(crate) fn ensure_run_start_pc_mapped(&self, pc: u64) -> Result<(), MwemuError> {
        if self.maps.get_mem_by_addr(pc).is_some() {
            return Ok(());
        }

        log::trace!("Cannot start emulation, pc pointing to unmapped area");
        Err(MwemuError::new(
            "program counter pointing to unmapped memory",
        ))
    }

    pub(crate) fn install_ctrlc_handler_if_enabled(&self) {
        if !self.enabled_ctrlc {
            return;
        }

        // With `--handle` + console mode, Ctrl-C should drop into the
        // interactive console at the next clean instruction boundary (the run
        // loop polls `ctrlc_console`), not stop the emulation. Without console
        // mode it just stops the run.
        let console_mode = self.cfg.console_enabled;
        let is_running = Arc::clone(&self.is_running);
        let ctrlc_console = Arc::clone(&self.ctrlc_console);
        let _ = ctrlc::set_handler(move || {
            if console_mode {
                ctrlc_console.store(1, atomic::Ordering::Relaxed);
            } else {
                is_running.store(0, atomic::Ordering::Relaxed);
            }
        });
    }

    pub(crate) fn reached_outer_run_limit(&self, pc: u64, end_addr: Option<u64>) -> Option<u64> {
        if let Some(end) = end_addr {
            if pc == end {
                return Some(pc);
            }
        }

        if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
            return Some(pc);
        }

        None
    }

    pub(crate) fn observe_loop_progress(
        &self,
        addr: u64,
        prev_addr: &mut u64,
        repeat_counter: &mut u32,
        looped: &mut Vec<u64>,
        infinite_loop_error: &'static str,
    ) -> Result<(), MwemuError> {
        if addr == *prev_addr {
            *repeat_counter = repeat_counter.saturating_add(1);
        } else {
            *repeat_counter = 0;
        }
        *prev_addr = addr;

        if *repeat_counter == 100 {
            log::trace!("infinite loop at 0x{:x}", addr);
            return Err(MwemuError::new(infinite_loop_error));
        }

        if self.cfg.loops {
            looped.push(addr);
            let count = looped.iter().filter(|&&seen| seen == addr).count() as u32;
            if count > 2 {
                log::trace!("    loop: {} iterations", count);
            }
        }

        Ok(())
    }

    pub(crate) fn trace_pre_step_state(&mut self, pos: u64) {
        if self.cfg.trace_regs && self.cfg.trace_filename.is_some() && pos >= self.cfg.trace_start {
            self.capture_pre_op();
        }

        if self.cfg.trace_reg {
            for reg in &self.cfg.reg_names {
                self.trace_specific_register(reg);
            }
        }

        if self.cfg.trace_flags {
            self.flags().print_trace(pos);
        }

        if self.cfg.trace_string {
            self.trace_string();
        }
    }

    pub(crate) fn check_runtime_limits(&mut self, pc: u64) -> Option<u64> {
        if let Some(max) = self.cfg.max_instructions {
            if self.instruction_count >= max {
                log::info!("max_instructions limit reached ({})", max);
                return Some(pc);
            }
        }

        if let Some(timeout) = self.cfg.timeout_secs {
            if self.instruction_count % 10000 == 0 {
                let elapsed = self.now.elapsed().as_secs_f64();
                if elapsed >= timeout {
                    log::info!("timeout reached ({:.1}s >= {:.1}s)", elapsed, timeout);
                    return Some(pc);
                }
            }
        }

        if let Some(max) = self.cfg.max_faults {
            if self.fault_count >= max {
                log::info!("max_faults limit reached ({})", max);
                return Some(pc);
            }
        }

        None
    }

    pub(crate) fn update_verbose_at(&mut self) {
        if let Some(vpos) = self.cfg.verbose_at {
            if vpos == self.pos {
                self.cfg.verbose = 3;
                self.cfg.trace_mem = true;
                self.cfg.trace_regs = true;
            }
        }
    }

    pub(crate) fn update_verbose_range(&mut self) {
        if self.cfg.verbose_start != 0 {
            let in_range = self.pos >= self.cfg.verbose_start
                && (self.cfg.verbose_end == 0 || self.pos <= self.cfg.verbose_end);
            if in_range {
                if self.cfg.verbose_range_saved.is_none() {
                    self.cfg.verbose_range_saved = Some(self.cfg.verbose);
                }
                self.cfg.verbose = 3;
            } else if let Some(orig) = self.cfg.verbose_range_saved.take() {
                self.cfg.verbose = orig;
            }
        }
    }
}
