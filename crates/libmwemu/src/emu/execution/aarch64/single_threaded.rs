use std::io::Write as _;
use std::sync::atomic;

use crate::color;
use crate::debug::console::Console;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine;
use crate::err::MwemuError;
use crate::serialization;
use crate::windows::constants;

use super::{Emu, assert_aarch64_arch};

impl Emu {
    /// AArch64 cached single-thread run loop. Owns AArch64-only behavior:
    /// fixed four-byte cache/decode progression, `Opcode::RET` recognition,
    /// AArch64 verbose instruction output, AArch64 register tracing, and
    /// `engine::aarch64::emulate_instruction`. Panics if the configured
    /// architecture is not AArch64.
    pub fn run_single_threaded_aarch64(
        &mut self,
        end_addr: Option<u64>,
    ) -> Result<u64, MwemuError> {
        assert_aarch64_arch(self, "run_single_threaded_aarch64");

        if self.process_terminated {
            return Err(MwemuError::new("process terminated (NtTerminateProcess)"));
        }
        self.ensure_run_start_pc_mapped(self.pc())?;

        self.is_running.store(1, atomic::Ordering::Relaxed);
        self.install_ctrlc_handler_if_enabled();

        // Cache booleans that drive hot-path gating. The config is effectively
        // immutable during a run, so evaluating these once up front lets the
        // inner loop skip entire branches when no observer/debug mode is on.
        let has_runtime_limits = self.cfg.max_instructions.is_some()
            || self.cfg.timeout_secs.is_some()
            || self.cfg.max_faults.is_some();
        let has_verbose_control = self.cfg.verbose_at.is_some() || self.cfg.verbose_start != 0;
        let has_pre_trace = self.cfg.trace_regs
            || self.cfg.trace_reg
            || self.cfg.trace_flags
            || self.cfg.trace_string;
        let has_post_trace = self.cfg.inspect || self.cfg.trace_regs;
        let has_execution_breakpoints = self.exp != u64::MAX
            || self.cfg.console2
            || !self.bp.addr.is_empty()
            || !self.bp.instruction.is_empty();

        let mut looped: Vec<u64> = Vec::new();
        let mut prev_addr: u64 = 0;
        let mut repeat_counter: u32 = 0;

        let mut aarch64_ins = yaxpeax_arm::armv8::a64::Instruction::default();
        let mut block: Vec<u8> = Vec::with_capacity(constants::BLOCK_LEN + 1);
        block.resize(constants::BLOCK_LEN, 0x0);

        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                let pc = self.pc();

                // Outer-loop limit checks: must run BEFORE attempting to fetch code,
                if let Some(limit_pc) = self.reached_outer_run_limit(pc, end_addr) {
                    std::hint::cold_path();
                    return Ok(limit_pc);
                }
                super::decode::ensure_instruction_cache_populated_aarch64(self, pc, &mut block)?;
                if !self.instruction_cache_can_decode() {
                    // Nothing decodable at this PC (e.g. unmapped page, garbage
                    // bytes that fail yaxpeax). Spinning `ensure_*` again would
                    // loop forever — surface the dead PC and let the caller
                    // (typically a `step()` user or `aarch64_call64`) decide.
                    log::trace!(
                        "aarch64 cached loop: no decodable instructions at pc=0x{pc:x}; bailing out"
                    );
                    return Ok(pc);
                }
                // Inner decode loop
                let mut sz: usize = 0;
                let mut addr: u64 = 0;

                let mut inner_running = true;
                let mut aarch64_decode_offset: u64 = 0;

                while inner_running {

                    // instruction boundary (not mid-REP), then re-fetch. Gated on
                    // the plain `enabled_ctrlc` bool so normal runs never touch
                    // the atomic on the per-instruction hot path.
                    if self.enabled_ctrlc
                        && self.rep.is_none()
                        && self.ctrlc_console.load(atomic::Ordering::Relaxed) == 1
                    {
                        self.ctrlc_console.store(0, atomic::Ordering::Relaxed);
                        Console::spawn_console(self);
                        break; // re-fetch from current PC (console may have stepped)
                    }

                    // Decode next instruction from cache
                    if self.rep.is_none() {
                        self.aarch64_instruction_cache()
                            .decode_out_aarch64_into(&mut aarch64_ins);
                        sz = 4;
                        addr = pc + aarch64_decode_offset;
                        aarch64_decode_offset += 4;

                        if end_addr.is_some() && Some(addr) == end_addr {
                            return Ok(self.pc());
                        }

                        if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                            return Ok(self.pc());
                        }
                    }
                    // the hot path.
                    if self.last_decoded.is_some() {
                        self.clear_last_decoded_instruction();
                    }
                    self.memory_operations.clear();

                    self.pos += 1;
                    self.instruction_count += 1;

                    // --- Limits ---
                    if has_runtime_limits {
                        if let Some(limit_pc) = self.check_runtime_limits(addr) {
                            return Ok(limit_pc);
                        }
                    }

                    // --- Verbose-at / verbose-range activation ---
                    if has_verbose_control {
                        self.update_verbose_at();
                        self.update_verbose_range();
                    }

                    let decoded: Option<DecodedInstruction> =
                        if self.needs_decoded_instruction_for_observers() {
                            Some(self.last_decoded_aarch64(addr, aarch64_ins))
                        } else {
                            None
                        };

                    // --- Exit position ---
                    if self.cfg.exit_position != 0 && self.pos == self.cfg.exit_position {
                        log::trace!("exit position reached");

                        if self.cfg.dump_on_exit && self.cfg.dump_filename.is_some() {
                            serialization::Serialization::dump(
                                self,
                                self.cfg.dump_filename.as_ref().unwrap(),
                            );
                        }

                        if self.cfg.trace_regs && self.cfg.trace_filename.is_some() {
                            self.trace_file
                                .as_ref()
                                .unwrap()
                                .flush()
                                .expect("failed to flush trace file");
                        }

                        return Ok(self.pc());
                    }

                    // --- Breakpoints ---
                    if has_execution_breakpoints
                        && ((self.exp != u64::MAX && self.exp == self.pos)
                            || self.bp.is_bp_instruction(self.pos)
                            || self.bp.is_bp(addr)
                            || (self.cfg.console2 && self.cfg.console_addr == addr))
                    {
                        if self.running_script {
                            return Ok(self.pc());
                        }

                        self.cfg.console2 = false;
                        if self.cfg.verbose >= 2 {
                            log::trace!("-------");
                            log::trace!("{} 0x{:x}: {}", self.pos, addr, aarch64_ins);
                        }
                        let pc_before_console = self.pc();
                        Console::spawn_console(self);
                        if self.force_break {
                            self.force_break = false;
                            break;
                        }
                        if self.pc() != pc_before_console {
                            break;
                        }
                    }

                    // --- Loop detection ---
                    if self.rep.is_none() {
                        self.observe_loop_progress(
                            addr,
                            &mut prev_addr,
                            &mut repeat_counter,
                            &mut looped,
                            "infinite loop found",
                        )?;
                    }

                    // --- Pre-instruction tracing ---
                    if has_pre_trace {
                        self.trace_pre_step_state(self.pos);
                    }

                    // --- Pre-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
                        let decoded =
                            decoded.unwrap_or_else(|| DecodedInstruction::AArch64(aarch64_ins));
                        let hook_pc = self.pc();
                        let skip = !hook_fn(self, hook_pc, &decoded, sz);
                        self.hooks.hook_on_pre_instruction = Some(hook_fn);
                        if skip {
                            inner_running = self.instruction_cache_can_decode();
                            continue;
                        }
                    }

                    // --- Entropy ---
                    if self.cfg.entropy && self.pos % 10000 == 0 {
                        self.update_entropy();
                    }

                    // --- Verbose output ---
                    // Use `show_instruction` so the line gets the same color
                    // as the post-mortem dump and the x86 path.
                    if self.cfg.verbose >= 2 {
                        let decoded =
                            decoded.unwrap_or_else(|| DecodedInstruction::AArch64(aarch64_ins));
                        self.show_instruction(color!("Cyan"), &decoded);
                    }

                    let should_stop_after_return = self.run_until_ret
                        && aarch64_ins.opcode == yaxpeax_arm::armv8::a64::Opcode::RET;

                    // --- Emulate ---
                    let emulation_ok = engine::aarch64::emulate_instruction(self, &aarch64_ins);
                    self.last_instruction_size = sz;

                    if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                        return Ok(self.pc());
                    }

                    // --- Post-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
                        let decoded =
                            decoded.unwrap_or_else(|| DecodedInstruction::AArch64(aarch64_ins));
                        let hook_pc = self.pc();
                        hook_fn(self, hook_pc, &decoded, sz, emulation_ok);
                        self.hooks.hook_on_post_instruction = Some(hook_fn);
                    }

                    // --- Post-execution tracing ---
                    if has_post_trace {
                        if self.cfg.inspect {
                            self.trace_memory_inspection();
                        }

                        if self.cfg.trace_regs
                            && self.cfg.trace_filename.is_some()
                            && self.pos >= self.cfg.trace_start
                        {
                            self.capture_post_op();
                            self.write_to_trace_file();
                        }
                    }

                    // --- Register trace ---
                    if self.cfg.trace_regs {
                        let regs = self.regs_aarch64();
                        log::trace!(
                            "  x0=0x{:x} x1=0x{:x} x2=0x{:x} x3=0x{:x} x8=0x{:x} x9=0x{:x} sp=0x{:x} lr=0x{:x}",
                            regs.x[0],
                            regs.x[1],
                            regs.x[2],
                            regs.x[3],
                            regs.x[8],
                            regs.x[9],
                            regs.sp,
                            regs.x[30]
                        );
                    }

                    // --- Failure handling ---
                    if !emulation_ok {
                        self.fault_count += 1;
                        if self.cfg.console_enabled {
                            Console::spawn_console(self);
                        } else if self.running_script {
                            return Ok(self.pc());
                        } else {
                            return Err(MwemuError::new(&format!(
                                "emulation error at pos = {} pc = 0x{:x}",
                                self.pos, addr
                            )));
                        }
                    }

                    // --- PC advance ---
                    if self.force_reload {
                        self.force_reload = false;
                        if should_stop_after_return {
                            return Ok(self.pc());
                        }
                        break; // break inner loop to re-fetch from new PC
                    }

                    self.advance_pc_aarch64(4);

                    // RET is fully emulated before run_until_ret stops at its architectural target.
                    if should_stop_after_return {
                        return Ok(self.pc());
                    }

                    if self.force_break {
                        self.force_break = false;
                        break;
                    }

                    // Check can_decode for next iteration
                    inner_running = self.instruction_cache_can_decode();
                } // end inner decode loop

                if self.is_api_run && self.is_break_on_api {
                    self.is_api_run = false;
                    break;
                }
            } // end running loop

            if self.is_break_on_api {
                return Ok(0);
            }

            self.is_running.store(1, atomic::Ordering::Relaxed);
            Console::spawn_console(self);
        } // end infinite loop
    } // end run_single_threaded_aarch64
}
