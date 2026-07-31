use std::io::Write as _;
use std::sync::atomic;

use iced_x86::{Decoder, DecoderOptions, Instruction};

use crate::debug::console::Console;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine;
use crate::err::MwemuError;
use crate::serialization;
use crate::syscall::windows::syscall64::memory as win_syscall64_memory;
use crate::windows::constants;

use super::Emu;

impl Emu {
    /// Emulate a single step from the current point (single-threaded implementation).
    /// this don't reset the emu.pos, that mark the number of emulated instructions and point to
    /// the current emulation moment.
    /// If you do a loop with emu.step() will have more control of the emulator but it will be
    /// slow.
    /// Is more convinient using run and run_to or even setting breakpoints.
    ///
    /// x86-only: this legacy helper predates the AArch64 support and accesses
    /// x86 register state directly. Use `step_x86`, `step_aarch64`, or the
    /// generic `step` dispatcher instead.
    #[deprecated(
        since = "0.1.0",
        note = "Use step() instead, which automatically handles threading and ISA selection"
    )]
    pub fn step_single_threaded(&mut self) -> bool {
        super::assert_x86_arch(self, "step_single_threaded");

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

        // code
        let rip = self.regs().rip;
        let code = match self.maps.get_mem_by_addr(rip) {
            Some(c) => c,
            None => {
                log::trace!(
                    "redirecting code flow to non maped address 0x{:x}",
                    self.regs().rip
                );
                Console::spawn_console(self);
                return false;
            }
        };

        // block
        let block = code.read_from(rip).to_vec(); // reduce code block for more speed

        // decoder
        let mut decoder;
        if self.cfg.is_x64() {
            decoder = Decoder::with_ip(64, &block, self.regs().rip, DecoderOptions::NONE);
        } else {
            decoder = Decoder::with_ip(32, &block, self.regs().get_eip(), DecoderOptions::NONE);
        }

        let ins = decoder.decode();
        let sz = ins.len();
        let addr = ins.ip();

        // clear
        self.memory_operations.clear();

        // format
        self.set_x86_instruction(Some(ins));

        let decoded = if self.needs_decoded_instruction_for_observers() {
            self.last_decoded_x86(addr, ins)
        } else {
            self.clear_last_decoded_instruction();
            // Build a one-shot decoded handle for hook delivery below without
            // permanently retaining it in the observer cache.
            DecodedInstruction::X86(ins)
        };

        // Run pre-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
            let decoded = if self.last_decoded.is_some() {
                decoded
            } else {
                DecodedInstruction::X86(ins)
            };
            let rip = self.regs().rip;
            let skip = !hook_fn(self, rip, &decoded, sz);
            self.hooks.hook_on_pre_instruction = Some(hook_fn);
            if skip {
                // update eip/rip
                self.advance_pc_x86(sz);
                return true; // skip instruction emulation
            }
        }
        // emulate
        let result_ok = engine::emulate_instruction(self, &ins, sz, true);
        //tracing::trace_instruction(self, self.pos);
        self.last_instruction_size = sz;

        // Run post-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
            let decoded = if self.last_decoded.is_some() {
                decoded
            } else {
                DecodedInstruction::X86(ins)
            };
            let rip = self.regs().rip;
            hook_fn(self, rip, &decoded, sz, result_ok);
            self.hooks.hook_on_post_instruction = Some(hook_fn);
        }

        // update eip/rip
        self.advance_pc_x86(sz);

        result_ok
    }

    /// x86-family cached single-thread run loop. Owns x86-only behavior:
    /// REP bulk path, REP state machine, x86 RET recognition, x86 --ssdt
    /// WinAPI shims, `RtlRaiseStatus` handling, and `ntdll_heap_list_walk_fixup`.
    /// Panics if the configured architecture is AArch64.
    pub fn run_single_threaded_x86(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        super::assert_x86_arch(self, "run_single_threaded_x86");
        let is_x64 = self.cfg.is_x64();

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

        let arch = if is_x64 { 64 } else { 32 };
        let mut x86_ins: Instruction = Instruction::default();
        let mut block: Vec<u8> = Vec::with_capacity(constants::BLOCK_LEN + 1);
        block.resize(constants::BLOCK_LEN, 0x0);

        loop {
            while self.is_running.load(atomic::Ordering::Relaxed) == 1 {
                let pc = self.pc();

                // Outer-loop limit checks: must run BEFORE attempting to fetch code,
                // otherwise PC sitting one past the end (e.g. after final loop iteration
                // under run_to) errors out as "unmapped" instead of cleanly stopping.
                if let Some(limit_pc) = self.reached_outer_run_limit(pc, end_addr) {
                    return Ok(limit_pc);
                }

                super::decode::ensure_instruction_cache_populated_x86(self, pc, &mut block, arch)?;

                // Inner decode loop
                let mut sz: usize = 0;
                let mut addr: u64 = 0;

                let mut inner_running = self.instruction_cache_can_decode();

                while inner_running {
                    // Ctrl-C (--handle): drop into the console at a clean
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
                        self.x86_instruction_cache()
                            .decode_out_x86_into(&mut x86_ins);
                        sz = x86_ins.len();
                        addr = x86_ins.ip();

                        if end_addr.is_some() && addr == end_addr.unwrap() {
                            return Ok(self.pc());
                        }

                        if self.max_pos.is_some() && self.pos >= self.max_pos.unwrap() {
                            return Ok(self.pc());
                        }
                    }
                    // saves two unconditional `Option` writes per instruction on
                    // the hot path. `clear_last_decoded_instruction_if_present`
                    // preserves the no-observer invariant that the final slot is
                    // empty (covered by test_run_no_observer_leaves_last_decoded_empty).
                    if self.last_decoded.is_some() {
                        self.clear_last_decoded_instruction();
                    }
                    self.memory_operations.clear();

                    // Bulk fast-path for REP string ops (rep stos/scas/movs/lods):
                    // executes the whole REP in one shot instead of one element
                    // per loop iteration. Only engages in pure-execution mode; in
                    // any observing mode it returns false and the per-element path
                    // below runs unchanged. Handles pos/instruction_count/rip.
                    if self.rep.is_none() && self.try_fast_rep_string(&x86_ins, sz) {
                        inner_running = self.instruction_cache_can_decode();
                        continue;
                    }

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
                            Some(self.last_decoded_x86(addr, x86_ins))
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

                    // --- API shims for --ssdt mode ----------------------------------
                    // When we run under `--ssdt`, kernel32/kernelbase code executes
                    // real PE bytes — which depends on a fully-initialised loader
                    // state we don't model perfectly. To unblock the most common
                    // entry points (LoadLibraryA, GetProcAddress, …) the moment we
                    // step *into* their first instruction we hand off to the native
                    // mwemu winapi64 implementation, then synthesise a `ret` so the
                    // caller proceeds without ever running the kernelbase body.
                    //
                    // The cheap pre-filter below avoids paying any per-instruction
                    // cost for the EXE itself (its PC is well below 0x7ff000000000)
                    // and lets us skip lookups for the >99% of fetches that don't
                    // land on a shimmed export.
                    let pc = self.pc();
                    if self.cfg.emulate_winapi && pc >= 0x7ff000000000 {
                        let shims = self.shim_table();
                        if shims.lla != 0 && pc == shims.lla {
                            crate::winapi::winapi64::kernel32::LoadLibraryA(self);
                            let ret_addr = self.stack_pop64(false).unwrap_or(0);
                            if self.cfg.verbose >= 1 {
                                log::trace!(
                                    "** {} kernelbase!LoadLibraryA shim → rax=0x{:x} ret=0x{:x}",
                                    self.pos,
                                    self.regs().rax,
                                    ret_addr,
                                );
                            }
                            self.regs_mut().rip = ret_addr;
                            self.pos += 1;
                            // Bust the decode cache so the outer loop refetches
                            // from the new RIP — `continue` alone only advances
                            // to the next instruction in the current cached
                            // block (the kernelbase body), which would happily
                            // run the byte AFTER the function entry.
                            inner_running = false;
                            continue;
                        }
                        if (shims.lpa != 0 && pc == shims.lpa)
                            || (shims.lpa2 != 0 && pc == shims.lpa2)
                        {
                            crate::winapi::winapi64::kernel32::GetProcAddress(self);
                            if self.cfg.verbose >= 1 {
                                log::trace!(
                                    "** {} kernelbase!GetProcAddress(ForCaller) shim → rax=0x{:x} pc=0x{:x}",
                                    self.pos,
                                    self.regs().rax,
                                    pc,
                                );
                            }
                            let ret_addr = self.stack_pop64(false).unwrap_or(0);
                            self.regs_mut().rip = ret_addr;
                            self.pos += 1;
                            inner_running = false;
                            continue;
                        }
                        // user32!MessageBoxA shim — under --ssdt we never run
                        // user32's DllMain, so its private globals (window
                        // class atoms, default heap, etc.) stay zeroed. Calling
                        // the real MessageBoxA body crashes at the first
                        // RtlAllocateHeap(NULL, …). Print the caption/text and
                        // return success so the caller proceeds.
                        if shims.mba != 0 && pc == shims.mba {
                            let text_ptr = self.regs().rdx;
                            let caption_ptr = self.regs().r8;
                            let text = self.maps.read_string(text_ptr);
                            let caption = self.maps.read_string(caption_ptr);
                            if self.cfg.verbose >= 1 {
                                log_red!(
                                    self,
                                    "** {} user32!MessageBoxA caption={:?} text={:?}",
                                    self.pos,
                                    caption,
                                    text,
                                );
                            }
                            // Print on the *real* stdout too so the operator
                            // sees the message even without verbose logging
                            // — this is the canonical signal that the demo
                            // shellcode reached its payload.
                            println!("MessageBoxA: [{}] {}", caption, text);
                            self.regs_mut().rax = 1; // IDOK
                            let ret_addr = self.stack_pop64(false).unwrap_or(0);
                            self.regs_mut().rip = ret_addr;
                            self.pos += 1;
                            inner_running = false;
                            continue;
                        }
                    }

                    // ntdll!RtlRaiseStatus entry (build-specific RVA 0x106fd0).
                    // On real Windows an unhandled exception path through here
                    // terminates the process. In our emulator
                    // `RtlRaiseNoncontinuableException` returns instead of dying,
                    // which traps the function in a self-recursion that eats the
                    // entire stack. Bail cleanly on the first entry — equivalent
                    // to "unhandled exception -> process terminated".
                    if addr == 0x180106fd0 {
                        let status = self.regs().get_ecx() as u32;
                        log::trace!(
                            "ntdll!RtlRaiseStatus(0x{:x}) at pos={} rsp=0x{:x} — terminating (no handler installed)",
                            status,
                            self.pos,
                            self.regs().rsp,
                        );
                        self.process_terminated = true;
                        self.is_running
                            .store(0, std::sync::atomic::Ordering::Relaxed);
                        self.force_break = true;
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
                            let output = decoded
                                .as_ref()
                                .map(|d| self.format_instruction(d))
                                .unwrap_or_default();
                            log::trace!("-------");
                            log::trace!("{} 0x{:x}: {}", self.pos, addr, output);
                        }
                        let rip_before_console = self.pc();
                        Console::spawn_console(self);
                        if self.force_break {
                            self.force_break = false;
                            break;
                        }
                        // If the console single-stepped (`enter`/`n` runs
                        // `emu.step()`), the instruction decoded above has
                        // already executed and `rip` moved on. Re-fetch from the
                        // new PC instead of falling through to `emulate` below —
                        // otherwise that stale instruction runs a second time.
                        if self.pc() != rip_before_console {
                            break;
                        }
                    }

                    // --- Loop detection (skip during REP) ---
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
                        let decoded = decoded.unwrap_or_else(|| DecodedInstruction::X86(x86_ins));
                        let hook_pc = self.pc();
                        let skip = !hook_fn(self, hook_pc, &decoded, sz);
                        self.hooks.hook_on_pre_instruction = Some(hook_fn);
                        if skip {
                            // Check can_decode for next iteration
                            inner_running = self.instruction_cache_can_decode();
                            continue;
                        }
                    }

                    // --- x86 REP prefix handling ---
                    if self.handle_x86_rep_pre_execution(x86_ins, sz) {
                        inner_running = self.instruction_cache_can_decode();
                        continue;
                    }

                    // --- Entropy ---
                    if self.cfg.entropy && self.pos % 10000 == 0 {
                        self.update_entropy();
                    }

                    // ntdll heap-list walk fixup — fires only under --ssdt to
                    // redirect empty LIST_ENTRY self-references. Gate at the
                    // call site so the function-call overhead disappears on the
                    // common (non-ssdt) path.
                    if self.cfg.emulate_winapi {
                        win_syscall64_memory::ntdll_heap_list_walk_fixup(self, &x86_ins, addr);
                    }

                    let should_stop_after_return =
                        self.run_until_ret && x86_ins.mnemonic() == iced_x86::Mnemonic::Ret;

                    // --- Emulate ---
                    let emulation_ok = engine::emulate_instruction(self, &x86_ins, sz, false);
                    self.last_instruction_size = sz;

                    if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                        return Ok(self.pc());
                    }

                    // --- x86 REP post-execution state machine ---
                    self.update_x86_rep_state_after_execution(x86_ins);

                    // --- Post-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
                        let decoded = decoded.unwrap_or_else(|| DecodedInstruction::X86(x86_ins));
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

                    if self.rep.is_none() {
                        self.advance_pc_x86(sz);
                    }

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
    } // end run_single_threaded_x86
}
