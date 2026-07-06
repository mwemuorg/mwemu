use std::io::Write as _;
use std::sync::atomic;

use iced_x86::{Decoder, DecoderOptions, Instruction};

use crate::color;
use crate::debug::console::Console;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::disassemble::InstructionCache;
use crate::emu::{ArchState, Emu};
use crate::err::MwemuError;

use crate::syscall::windows::syscall64::memory as win_syscall64_memory;
use crate::windows::peb::peb64;
use crate::{engine, serialization, windows::constants};

// API-shim entry-point cache. Resolved ONCE on the first time execution
// enters the loader-DLL VA range; thereafter every per-instruction
// lookup is a single thread-local Cell read with no locks or hashing.
// `0xFFFF_FFFF_FFFF_FFFF` is the "tried-and-resolved" sentinel for
// symbols whose module is loaded but the symbol itself is missing — we
// still want to skip the export-walk on every subsequent instruction.
use std::cell::Cell;
thread_local! {
    static SHIM_TABLE: Cell<Option<ShimTable>> = const { Cell::new(None) };
}

#[derive(Clone, Copy, Default)]
pub(super) struct ShimTable {
    pub lla: u64,
    pub lpa: u64,
    pub lpa2: u64,
    pub mba: u64,
}

impl Emu {
    #[inline]
    pub(super) fn shim_table(&mut self) -> ShimTable {
        let cached = SHIM_TABLE.with(|c| c.get());
        // kernelbase symbols are always present once kernelbase is loaded
        // (which happens during early `--ssdt` setup). user32 is loaded
        // on-demand by the LoadLibraryA shim, so mba may resolve to 0 on
        // the first pass — refresh it lazily once user32 appears.
        if let Some(t) = cached {
            if t.mba != 0 || self.maps.get_map_by_name("user32.pe").is_none() {
                return t;
            }
            // user32 is loaded now but mba was 0 — re-resolve just mba.
            let mba = crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "user32.dll",
                "MessageBoxA",
            );
            let new = ShimTable { mba, ..t };
            SHIM_TABLE.with(|c| c.set(Some(new)));
            if self.cfg.verbose >= 1 {
                log::trace!("shim table mba resolved: 0x{:x}", mba);
            }
            return new;
        }
        let t = ShimTable {
            lla: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "kernelbase.dll",
                "LoadLibraryA",
            ),
            lpa: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "kernelbase.dll",
                "GetProcAddress",
            ),
            lpa2: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "kernelbase.dll",
                "GetProcAddressForCaller",
            ),
            mba: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "user32.dll",
                "MessageBoxA",
            ),
        };
        SHIM_TABLE.with(|c| c.set(Some(t)));
        if self.cfg.verbose >= 1 {
            log::trace!(
                "shim table resolved: LLA=0x{:x} GPA=0x{:x} GPA-FC=0x{:x} MBA=0x{:x}",
                t.lla,
                t.lpa,
                t.lpa2,
                t.mba,
            );
        }
        t
    }
}

mod control;
mod decode;
mod multithreaded;
mod rep;

macro_rules! round_to {
    ($num:expr, $dec:expr) => {{
        let factor = 10f64.powi($dec);
        ($num * factor).round() / factor
    }};
}

/// Maximum nesting depth for emulator-driven call64/call32/linux_call64 invocations.
/// Normal emulated `call` instructions do not increment this counter — only explicit
/// host-side calls (loader bootstrap, TLS callbacks, etc.) do. A depth beyond this
/// most likely indicates a bug in the emulator infrastructure.
const MAX_CALL_DEPTH: u32 = 32;

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
    fn needs_decoded_instruction_for_observers(&self, _is_aarch64: bool) -> bool {
        self.hooks.hook_on_pre_instruction.is_some()
            || self.hooks.hook_on_post_instruction.is_some()
            || self.needs_trace_file_instruction()
            || self.cfg.verbose >= 2
    }

    #[inline]
    fn current_decoded_instruction(
        is_aarch64: bool,
        x86_ins: Instruction,
        aarch64_ins: yaxpeax_arm::armv8::a64::Instruction,
    ) -> DecodedInstruction {
        if is_aarch64 {
            DecodedInstruction::AArch64(aarch64_ins)
        } else {
            DecodedInstruction::X86(x86_ins)
        }
    }

    #[inline]
    fn clear_last_decoded_instruction(&mut self) {
        self.last_decoded = None;
        self.last_decoded_addr = 0;
    }

    #[inline]
    fn ensure_last_decoded_instruction(
        &mut self,
        decoded: &mut Option<DecodedInstruction>,
        is_aarch64: bool,
        addr: u64,
        x86_ins: Instruction,
        aarch64_ins: yaxpeax_arm::armv8::a64::Instruction,
    ) -> DecodedInstruction {
        let decoded = *decoded.get_or_insert_with(|| {
            Self::current_decoded_instruction(is_aarch64, x86_ins, aarch64_ins)
        });
        self.last_decoded = Some(decoded);
        self.last_decoded_addr = addr;
        decoded
    }

    /// Decode and execute one instruction at the current PC.
    /// Returns (instruction_size, emulation_ok).
    /// Dispatches to x86 or aarch64 decode/execute internally.
    pub fn decode_and_execute(&mut self) -> (usize, bool) {
        let pc = self.pc();

        // Fetch code
        let code = match self.maps.get_mem_by_addr(pc) {
            Some(c) => c,
            None => {
                log::trace!("code flow to unmapped address 0x{:x}", pc);
                Console::spawn_console(self);
                return (0, false);
            }
        };

        self.memory_operations.clear();

        if self.cfg.arch.is_aarch64() {
            // --- AArch64 decode & execute ---
            let block = code.read_bytes(pc, 4);
            if block.len() < 4 {
                log::warn!("aarch64: cannot read 4 bytes at 0x{:x}", pc);
                return (0, false);
            }

            let decoder = yaxpeax_arm::armv8::a64::InstDecoder::default();
            let mut reader = yaxpeax_arch::U8Reader::new(block);
            let ins = match yaxpeax_arch::Decoder::decode(&decoder, &mut reader) {
                Ok(ins) => ins,
                Err(e) => {
                    log::warn!("aarch64: decode error at 0x{:x}: {:?}", pc, e);
                    return (0, false);
                }
            };

            if self.cfg.verbose >= 2 {
                log::trace!("{} 0x{:x}: {}", self.pos, pc, ins);
            }

            self.last_decoded = Some(DecodedInstruction::AArch64(ins));
            self.last_decoded_addr = pc;

            // Pre-instruction hook
            if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
                let skip = !hook_fn(self, pc, &self.last_decoded.unwrap(), 4);
                self.hooks.hook_on_pre_instruction = Some(hook_fn);
                if skip {
                    return (4, true); // skip instruction emulation but report as successful
                }
            }

            let result_ok = engine::aarch64::emulate_instruction(self, &ins);
            self.last_instruction_size = 4;

            // Post-instruction hook
            if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
                hook_fn(self, pc, &self.last_decoded.unwrap(), 4, result_ok);
                self.hooks.hook_on_post_instruction = Some(hook_fn);
            }

            (4, result_ok)
        } else {
            // --- x86 decode & execute ---
            let block = code.read_from(pc).to_vec();
            let mut decoder = if self.cfg.is_x64() {
                Decoder::with_ip(64, &block, pc, DecoderOptions::NONE)
            } else {
                Decoder::with_ip(32, &block, pc, DecoderOptions::NONE)
            };

            let ins = decoder.decode();
            let sz = ins.len();
            let position = decoder.position();

            self.set_x86_instruction(Some(ins));
            self.set_x86_decoder_position(position);
            self.last_decoded = Some(DecodedInstruction::X86(ins));
            self.last_decoded_addr = pc;

            // Pre-instruction hook
            if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
                let skip = !hook_fn(self, pc, &self.last_decoded.unwrap(), sz);
                self.hooks.hook_on_pre_instruction = Some(hook_fn);
                if skip {
                    return (sz, true); // skip instruction emulation but report as successful
                }
            }

            let result_ok = engine::emulate_instruction(self, &ins, sz, true);
            self.last_instruction_size = sz;

            // Post-instruction hook
            if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
                hook_fn(self, pc, &self.last_decoded.unwrap(), sz, result_ok);
                self.hooks.hook_on_post_instruction = Some(hook_fn);
            }

            (sz, result_ok)
        }
    }

    /// Advance the program counter by `sz` bytes.
    /// Respects force_reload (branch already set PC).
    /// Dispatches to RIP, EIP, or PC based on architecture.
    #[inline]
    pub fn advance_pc(&mut self, sz: usize) {
        if self.force_reload {
            self.force_reload = false;
        } else if self.cfg.arch.is_aarch64() {
            self.regs_aarch64_mut().pc += sz as u64;
        } else if self.cfg.is_x64() {
            self.regs_mut().rip += sz as u64;
        } else {
            let eip = self.regs().get_eip() + sz as u64;
            self.regs_mut().set_eip(eip);
        }
    }

    /// Call a 32bits function at addr, passing argument in an array of u64 but will cast to u32.
    /// The calling convention is stack, like winapi32.
    pub fn call32(&mut self, addr: u64, args: &[u32]) -> Result<u32, MwemuError> {
        if addr == self.regs().get_eip() {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call32, change eip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }
        let orig_stack = self.regs().get_esp();
        for arg in args.iter().rev() {
            self.stack_push32(*arg);
        }
        let ret_addr = self.regs().get_eip();
        self.stack_push32(ret_addr as u32);
        self.regs_mut().set_eip(addr);
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;
        self.regs_mut().set_esp(orig_stack);
        Ok(self.regs().get_eax() as u32)
    }

    /// Call 64bits function at addr using Microsoft x64 ABI, passing argument in an array of u64.
    /// The calling convention is registers rcx/rdx/r8/r9 and then stack. Like windows64.
    /// Dont use for linux64 syscall like convention, for this is linux_call64()
    pub fn call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }

        let n = args.len();
        if n >= 1 {
            self.regs_mut().rcx = args[0];
        }
        if n >= 2 {
            self.regs_mut().rdx = args[1];
        }
        if n >= 3 {
            self.regs_mut().r8 = args[2];
        }
        if n >= 4 {
            self.regs_mut().r9 = args[3];
        }

        // stack pointer backup, for restoring when function returns.
        let orig_stack = self.regs().rsp;

        // padding
        let extra_args = if n > 4 { (n - 4) * 8 } else { 0 };
        let total = extra_args + 32 + 8;
        let padding = (16 - (self.regs().rsp as usize + total) % 16) % 16;
        self.regs_mut().rsp -= padding as u64;

        // shadow space (32bits)
        for _ in 0..4 {
            self.stack_push64(0);
        }

        // stack parameters
        if n > 4 {
            for arg in args.iter().skip(4).rev() {
                self.stack_push64(*arg);
            }
        }

        // return address
        let ret_addr = self.regs().rip;
        self.stack_push64(ret_addr);

        // trigger function
        self.regs_mut().rip = addr;

        // emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }

    /// Call a 64bits function at addr, passing arguments in an array of u64.
    /// The calling convention is registers RDI, RSI, RDX, RCX, R8, R9 and then stack. Like linux64.
    pub fn linux_call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }

        let n = args.len();
        if n >= 1 {
            self.regs_mut().rdi = args[0];
        }
        if n >= 2 {
            self.regs_mut().rsi = args[1];
        }
        if n >= 3 {
            self.regs_mut().rdx = args[2];
        }
        if n >= 4 {
            self.regs_mut().rcx = args[3];
        }
        if n >= 5 {
            self.regs_mut().r8 = args[4];
        }
        if n >= 6 {
            self.regs_mut().r9 = args[5];
        }

        // stack pointer backup, for restoring when function returns.
        let orig_stack = self.regs().rsp;

        // padding
        let extra_args = if n > 6 { (n - 6) * 8 } else { 0 };
        let total = extra_args + 8;
        let padding = (16 - (self.regs().rsp as usize + total) % 16) % 16;
        self.regs_mut().rsp -= padding as u64;

        // stack parameters
        if n > 6 {
            for arg in args.iter().skip(6).rev() {
                self.stack_push64(*arg);
            }
        }

        // return address
        let ret_addr = self.regs().rip;
        self.stack_push64(ret_addr);

        // trigger function
        self.regs_mut().rip = addr;

        // emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }

    /// Call a 64-bit function using AArch64 AAPCS64 calling convention.
    /// Args in x0-x7, return value in x0, LR = return address.
    pub fn aarch64_call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        let current_pc = self.pc();
        if addr == current_pc {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting aarch64_call64, change pc.",
                ));
            } else {
                self.set_pc(0);
            }
        }

        // Load args into x0-x7
        let n = args.len().min(8);
        for i in 0..n {
            self.regs_aarch64_mut().x[i] = args[i];
        }
        if args.len() > 8 {
            log::warn!("aarch64_call64: more than 8 args not yet supported");
        }

        // Save SP
        let orig_sp = self.regs_aarch64().sp;

        // 16-byte align SP
        let sp = self.regs_aarch64().sp;
        let aligned_sp = sp & !0xF;
        self.regs_aarch64_mut().sp = aligned_sp;

        // Set return address in LR (x30)
        let ret_addr = self.pc();
        self.regs_aarch64_mut().x[30] = ret_addr;

        // Jump to target
        self.set_pc(addr);

        // Emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // Restore SP and return x0
        self.regs_aarch64_mut().sp = orig_sp;
        Ok(self.regs_aarch64().x[0])
    }

    /// Start emulation until a ret instruction is found.
    /// It will return the address or MwemuError.
    #[inline]
    pub fn run_until_ret(&mut self) -> Result<u64, MwemuError> {
        self.run_until_ret = true;
        self.run(None)
    }

    /// Emulate a single step from the current point.
    /// Works for both x86 and aarch64. Handles hooks, threading, exit_position.
    #[allow(deprecated)]
    pub fn step(&mut self) -> bool {
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

    pub fn update_entropy(&mut self) {
        let prev_entropy = self.entropy;

        let mem = match self.maps.get_mem_by_addr(self.pc()) {
            Some(n) => n,
            None => {
                self.entropy = 0.0;
                if self.entropy != prev_entropy {
                    log::trace!(
                        "{}:0x{:x} entropy changed {} ->  {}",
                        self.pos,
                        self.pc(),
                        prev_entropy,
                        self.entropy
                    );
                }
                return;
            }
        };

        let data = mem.get_bytes();

        if data.is_empty() {
            self.entropy = 0.0;
            if self.entropy != prev_entropy {
                log::trace!(
                    "{}:0x{:x} entropy changed {} ->  {}",
                    self.pos,
                    self.pc(),
                    prev_entropy,
                    self.entropy
                );
            }
            return;
        }

        let mut counts = [0usize; 256];
        for &b in data {
            counts[b as usize] += 1;
        }
        let len = data.len() as f64;
        self.entropy = round_to!(
            counts
                .iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / len;
                    -p * p.log2()
                })
                .sum::<f64>(),
            3
        );

        if self.entropy != prev_entropy {
            log::trace!(
                "{}:0x{:x} entropy changed {} ->  {}",
                self.pos,
                self.pc(),
                prev_entropy,
                self.entropy
            );
        }
    }

    /// Emulate a single step from the current point (single-threaded implementation).
    /// this don't reset the emu.pos, that mark the number of emulated instructions and point to
    /// the current emulation moment.
    /// If you do a loop with emu.step() will have more control of the emulator but it will be
    /// slow.
    /// Is more convinient using run and run_to or even setting breakpoints.
    #[deprecated(
        since = "0.1.0",
        note = "Use step() instead, which automatically handles threading"
    )]
    pub fn step_single_threaded(&mut self) -> bool {
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

        // get first instruction from iterator
        let ins = decoder.decode();
        let sz = ins.len();
        let addr = ins.ip();
        let position = decoder.position();

        // clear
        self.memory_operations.clear();

        // format
        self.set_x86_instruction(Some(ins));
        self.set_x86_decoder_position(position);

        let mut decoded: Option<DecodedInstruction> = None;
        if self.needs_decoded_instruction_for_observers(false) {
            self.ensure_last_decoded_instruction(
                &mut decoded,
                false,
                addr,
                ins,
                Default::default(),
            );
        } else {
            self.clear_last_decoded_instruction();
        }

        // Run pre-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
            let decoded = self.ensure_last_decoded_instruction(
                &mut decoded,
                false,
                addr,
                ins,
                Default::default(),
            );
            let rip = self.regs().rip;
            let skip = !hook_fn(self, rip, &decoded, sz);
            self.hooks.hook_on_pre_instruction = Some(hook_fn);
            if skip {
                // update eip/rip
                if self.force_reload {
                    self.force_reload = false;
                } else if self.cfg.is_x64() {
                    self.regs_mut().rip += sz as u64;
                } else {
                    let eip = self.regs().get_eip() + sz as u64;
                    self.regs_mut().set_eip(eip);
                }
                return true; // skip instruction emulation
            }
        }
        // emulate
        let result_ok = engine::emulate_instruction(self, &ins, sz, true);
        //tracing::trace_instruction(self, self.pos);
        self.last_instruction_size = sz;

        // Run post-instruction hook
        if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
            let decoded = self.ensure_last_decoded_instruction(
                &mut decoded,
                false,
                addr,
                ins,
                Default::default(),
            );
            let rip = self.regs().rip;
            hook_fn(self, rip, &decoded, sz, result_ok);
            self.hooks.hook_on_post_instruction = Some(hook_fn);
        }

        // update eip/rip
        if self.force_reload {
            self.force_reload = false;
        } else if self.cfg.is_x64() {
            self.regs_mut().rip += sz as u64;
        } else {
            let eip = self.regs().get_eip() + sz as u64;
            self.regs_mut().set_eip(eip);
        }

        result_ok
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
    #[allow(deprecated)]
    pub fn run(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        // Reset instruction cache for the active architecture
        match &mut self.arch_state {
            ArchState::X86 {
                instruction_cache, ..
            } => *instruction_cache = InstructionCache::new(),
            ArchState::AArch64 {
                instruction_cache, ..
            } => *instruction_cache = InstructionCache::new(),
        }
        if !self.os.is_linux()
            && self.cfg.arch.is_64bits()
            && self.cfg.ssdt_use_ldr_initialize_thunk
            && self.maps.get_map_by_name("peb").is_some()
        {
            peb64::ensure_peb_system_dependent_07(self);
        }
        if !self.cfg.arch.is_aarch64() {
            *self.x86_instruction_cache() = InstructionCache::new();
        }
        if self.cfg.enable_threading && self.threads.len() > 1 {
            self.run_multi_threaded(end_addr)
        } else {
            self.run_single_threaded(end_addr)
        }
    }

    /// Unified single-threaded emulation loop for both x86 and aarch64.
    ///
    /// Both architectures share identical structure: outer cache-miss → inner
    /// cache-hit decode loop.  The only x86-specific section is REP prefix
    /// handling (~30 lines), guarded by `decoded.is_x86()`.
    #[deprecated(
        since = "0.1.0",
        note = "Use run() instead, which automatically handles threading"
    )]
    pub fn run_single_threaded(&mut self, end_addr: Option<u64>) -> Result<u64, MwemuError> {
        let is_aarch64 = self.cfg.arch.is_aarch64();

        if self.process_terminated {
            return Err(MwemuError::new("process terminated (NtTerminateProcess)"));
        }
        self.ensure_run_start_pc_mapped(self.pc())?;

        self.is_running.store(1, atomic::Ordering::Relaxed);
        self.install_ctrlc_handler_if_enabled();

        let mut looped: Vec<u64> = Vec::new();
        let mut prev_addr: u64 = 0;
        let mut repeat_counter: u32 = 0;

        let arch = if self.cfg.is_x64() { 64 } else { 32 };
        let mut x86_ins: Instruction = Instruction::default();
        let mut aarch64_ins = yaxpeax_arm::armv8::a64::Instruction::default();
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

                self.ensure_instruction_cache_populated(pc, &mut block, arch, is_aarch64)?;

                // Inner decode loop
                let mut sz: usize = 0;
                let mut addr: u64 = 0;

                let mut inner_running = self.instruction_cache_can_decode();
                let mut aarch64_decode_offset: u64 = 0;

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
                    if is_aarch64 {
                        if self.rep.is_none() {
                            match &mut self.arch_state {
                                ArchState::AArch64 {
                                    instruction_cache,
                                    instruction,
                                    ..
                                } => {
                                    instruction_cache.decode_out(&mut aarch64_ins);
                                    *instruction = Some(aarch64_ins);
                                }
                                _ => unreachable!(),
                            }
                            sz = 4;
                            addr = pc + aarch64_decode_offset;
                            aarch64_decode_offset += 4;
                        }
                    } else {
                        if self.rep.is_none() {
                            match &mut self.arch_state {
                                ArchState::X86 {
                                    instruction_cache, ..
                                } => {
                                    instruction_cache.decode_out(&mut x86_ins);
                                }
                                _ => unreachable!(),
                            }
                            sz = x86_ins.len();
                            addr = x86_ins.ip();

                            if end_addr.is_some() && Some(addr) == end_addr {
                                return Ok(self.pc());
                            }

                            if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                                return Ok(self.pc());
                            }
                        }
                        self.set_x86_instruction(Some(x86_ins));
                        match &self.arch_state {
                            ArchState::X86 {
                                instruction_cache, ..
                            } => {
                                self.set_x86_decoder_position(
                                    instruction_cache.current_instruction_slot,
                                );
                            }
                            _ => unreachable!(),
                        }
                    }

                    // aarch64 end_addr / max_pos checks (x86 checked above during decode)
                    if is_aarch64 {
                        if let Some(end) = end_addr {
                            if addr == end {
                                return Ok(self.pc());
                            }
                        }
                        if self.max_pos.is_some() && Some(self.pos) >= self.max_pos {
                            return Ok(self.pc());
                        }
                    }

                    self.clear_last_decoded_instruction();
                    self.memory_operations.clear();

                    // Bulk fast-path for REP string ops (rep stos/scas/movs/lods):
                    // executes the whole REP in one shot instead of one element
                    // per loop iteration. Only engages in pure-execution mode; in
                    // any observing mode it returns false and the per-element path
                    // below runs unchanged. Handles pos/instruction_count/rip.
                    if !is_aarch64 && self.rep.is_none() && self.try_fast_rep_string(&x86_ins, sz) {
                        inner_running = self.instruction_cache_can_decode();
                        continue;
                    }

                    self.pos += 1;
                    self.instruction_count += 1;

                    // --- Limits ---
                    if let Some(limit_pc) = self.check_runtime_limits(self.pc()) {
                        return Ok(limit_pc);
                    }

                    self.update_verbose_at();

                    // --- verbose_range activation (-X a,b) ---
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

                    let mut decoded: Option<DecodedInstruction> = None;
                    if self.needs_decoded_instruction_for_observers(is_aarch64) {
                        self.ensure_last_decoded_instruction(
                            &mut decoded,
                            is_aarch64,
                            addr,
                            x86_ins,
                            aarch64_ins,
                        );
                    }

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

                    // DEBUG: trace shellcode resolver checkpoints
                    match addr {
                        0x14000116a => {
                            log::trace!(
                                "DEBUG @0x{:x} after LoadLibraryA('user32.dll') rax=0x{:x}",
                                addr,
                                self.regs().rax
                            );
                            // Dump LDR chain at this point to see if user32 is linked.
                            let peb_base = self.maps.get_mem("peb").get_base();
                            let ldr = self.maps.read_qword(peb_base + 0x18).unwrap_or(0);
                            let sentinel = ldr + 0x20;
                            let mut cur = self.maps.read_qword(sentinel).unwrap_or(0);
                            let mut i = 0;
                            while cur != 0 && cur != sentinel && i < 24 {
                                let entry = cur.wrapping_sub(0x10);
                                let dll_base = self.maps.read_qword(entry + 0x30).unwrap_or(0);
                                let name_len =
                                    self.maps.read_word(entry + 0x58).unwrap_or(0) as u64;
                                let name_buf = self.maps.read_qword(entry + 0x58 + 8).unwrap_or(0);
                                let mut s = String::new();
                                let mut j = 0u64;
                                while j < name_len.min(128) {
                                    let w = self.maps.read_word(name_buf + j).unwrap_or(0);
                                    if w == 0 {
                                        break;
                                    }
                                    s.push(char::from_u32(w as u32).unwrap_or('?'));
                                    j += 2;
                                }
                                log::trace!(
                                    "  DEBUG_POST_LL [{}] entry=0x{:x} DllBase=0x{:x} name='{}'",
                                    i,
                                    entry,
                                    dll_base,
                                    s
                                );
                                cur = self.maps.read_qword(cur).unwrap_or(0);
                                i += 1;
                            }
                        }
                        0x140001188 => log::trace!(
                            "DEBUG @0x{:x} after GetProcAddress(user32, 'MessageBoxA') rax=0x{:x}",
                            addr,
                            self.regs().rax
                        ),
                        0x140001186 => log::trace!(
                            "DEBUG @0x{:x} call rax(GetProcAddress) rcx=0x{:x} rdx=0x{:x} rax(target)=0x{:x}",
                            addr,
                            self.regs().rcx,
                            self.regs().rdx,
                            self.regs().rax
                        ),
                        0x140001168 => {
                            log::trace!(
                                "DEBUG @0x{:x} call rax(LoadLibraryA) rcx=0x{:x} rax(target)=0x{:x}",
                                addr,
                                self.regs().rcx,
                                self.regs().rax
                            );
                            let thunk = self.regs().rax;
                            let iat_ptr = thunk.wrapping_add(7).wrapping_add(0x610e1);
                            let bound = self.maps.read_qword(iat_ptr).unwrap_or(0);
                            log::trace!(
                                "DEBUG kernel32!LoadLibraryA IAT[0x{:x}] = 0x{:x}",
                                iat_ptr,
                                bound
                            );
                        }
                        // After kernelbase!LoadLibraryA's pre-check call returns
                        0x7ff000139eca => log::trace!(
                            "DEBUG kernelbase!LoadLibraryA pre-check returned eax=0x{:x}",
                            self.regs().get_eax() as u32
                        ),
                        // The "main" LoadLibraryExW call
                        0x7ff000139eda => log::trace!(
                            "DEBUG kernelbase!LoadLibraryA about to call LoadLibraryExW rcx=0x{:x}",
                            self.regs().rcx
                        ),
                        // The ret of kernelbase!LoadLibraryA
                        0x7ff000139eee => log::trace!(
                            "DEBUG kernelbase!LoadLibraryA RET rax=0x{:x}",
                            self.regs().rax
                        ),
                        // The "error" path
                        0x7ff000178b32 => log::trace!(
                            "DEBUG kernelbase!LoadLibraryA took error path @0xb6b32 (returns 0)"
                        ),
                        // LoadLibraryExW internals (kernelbase+0x2bbb0):
                        0x7ff0000edbcd => log::trace!(
                            "DEBUG LoadLibraryExW after BasepConvert/Normalize call eax=0x{:x}",
                            self.regs().get_eax() as u32
                        ),
                        0x7ff0000edc04 => {
                            log::trace!("DEBUG LoadLibraryExW jumped to error path 0x2bc04")
                        }
                        0x7ff0000edbe1 => log::trace!(
                            "DEBUG LoadLibraryExW worker returned rax=0x{:x}",
                            self.regs().rax
                        ),
                        // Worker (BasepLoadLibraryExW) branch points (kernelbase+0x235a0):
                        0x7ff0000e571b => log::trace!("DEBUG worker took early-error path 0x2371b"),
                        0x7ff000161fb8 => log::trace!(
                            "DEBUG worker took error path 0x9ffb8 (after first IAT call signed)"
                        ),
                        0x7ff000161fd8 => log::trace!(
                            "DEBUG worker took error path 0x9ffd8 (after RtlGetFullPathName signed)"
                        ),
                        0x7ff0000e970b => log::trace!("DEBUG worker took success path 0x2370b"),
                        0x7ff0000e96b1 => {
                            log::trace!("DEBUG worker xor eax, eax (return 0) reached")
                        }
                        // After internal calls
                        0x7ff0000e55ed => log::trace!(
                            "DEBUG worker after first IAT call eax=0x{:x} [local=0x{:x}_0x{:x}]",
                            self.regs().get_eax() as u32,
                            self.maps
                                .read_word(self.regs().rbp.wrapping_sub(0x10))
                                .unwrap_or(0),
                            self.maps
                                .read_qword(self.regs().rbp.wrapping_sub(0x8))
                                .unwrap_or(0)
                        ),
                        // Right BEFORE the first IAT call: read the IAT to find which function we're invoking
                        0x7ff0000e55e1 => {
                            // call qword ptr [rip + 0x19c400] — 7-byte instruction
                            let iat_ptr = 0x7ff0000e55e1u64.wrapping_add(7).wrapping_add(0x19c400);
                            let bound = self.maps.read_qword(iat_ptr).unwrap_or(0);
                            log::trace!(
                                "DEBUG worker about to call IAT[0x{:x}] = 0x{:x} (rcx=0x{:x} rdx=0x{:x})",
                                iat_ptr,
                                bound,
                                self.regs().rcx,
                                self.regs().rdx
                            );
                        }
                        0x7ff0000e565b => log::trace!(
                            "DEBUG worker after RtlGetFullPathName_UEx call eax=0x{:x}",
                            self.regs().get_eax() as u32
                        ),
                        0x7ff0000e5680 => log::trace!(
                            "DEBUG worker after call 0x20f50 (real loader) eax=0x{:x}",
                            self.regs().get_eax() as u32
                        ),
                        // The "flags == 0" branch (most common, our case):
                        0x7ff0000e56fb => {
                            let iat_ptr = 0x7ff0000e56fbu64.wrapping_add(7).wrapping_add(0x19c92e);
                            let bound = self.maps.read_qword(iat_ptr).unwrap_or(0);
                            log::trace!(
                                "DEBUG worker flags=0 branch: about to call IAT[0x{:x}] = 0x{:x} (LdrLoadDll-like)",
                                iat_ptr,
                                bound
                            );
                            log::trace!(
                                "DEBUG   rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x}",
                                self.regs().rcx,
                                self.regs().rdx,
                                self.regs().r8,
                                self.regs().r9
                            );
                        }
                        0x7ff0000e5702 => log::trace!(
                            "DEBUG worker after LdrLoadDll call eax=0x{:x} [rbp+0x38]=0x{:x}",
                            self.regs().get_eax() as u32,
                            self.maps
                                .read_qword(self.regs().rbp.wrapping_add(0x38))
                                .unwrap_or(0)
                        ),
                        // Right before the RtlRaiseStatus call inside the unwind/error function
                        0x1800c3e18 => {
                            static ONCE: std::sync::atomic::AtomicBool =
                                std::sync::atomic::AtomicBool::new(false);
                            if !ONCE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                                log::trace!(
                                    "DEBUG @0xc3e18 about to call RtlRaiseStatus rbx=0x{:x} (status) rdi=0x{:x} rsi=0x{:x}",
                                    self.regs().rbx,
                                    self.regs().rdi,
                                    self.regs().rsi
                                );
                                // Walk return addresses upward to identify the call chain
                                for i in 0..16u64 {
                                    let addr = self.regs().rsp.wrapping_add(i * 8);
                                    if let Some(v) = self.maps.read_qword(addr) {
                                        if (0x180000000..0x180400000).contains(&v)
                                            || (0x7ff000000000..0x7ff800000000).contains(&v)
                                        {
                                            let m = self.maps.get_addr_name(v).unwrap_or("?");
                                            log::trace!("  rsp+0x{:x}: 0x{:x} ({})", i * 8, v, m);
                                        }
                                    }
                                }
                                // Dump 64 bytes at rdi (likely import-by-name struct)
                                let rdi = self.regs().rdi;
                                let mut hex = String::new();
                                let mut ascii = String::new();
                                for j in 0..64u64 {
                                    let b = self.maps.read_byte(rdi + j).unwrap_or(0);
                                    hex.push_str(&format!("{:02x} ", b));
                                    ascii.push(if (0x20..=0x7e).contains(&b) {
                                        b as char
                                    } else {
                                        '.'
                                    });
                                }
                                log::trace!("  rdi[0..64] hex: {}", hex);
                                log::trace!("  rdi[0..64] asc: {}", ascii);
                                // Also dump 64 bytes at rsi (ntdll context)
                                let rsi = self.regs().rsi;
                                let mut hex2 = String::new();
                                for j in 0..64u64 {
                                    let b = self.maps.read_byte(rsi + j).unwrap_or(0);
                                    hex2.push_str(&format!("{:02x} ", b));
                                }
                                log::trace!("  rsi[0..64] hex: {}", hex2);
                                // Deref the buffer pointers from the UNICODE_STRING-like structure at rdi.
                                let buf1 = self.maps.read_qword(rdi + 8).unwrap_or(0);
                                let len1 = self.maps.read_word(rdi).unwrap_or(0) as u64;
                                if buf1 != 0 && self.maps.is_mapped(buf1) {
                                    let mut s = String::new();
                                    for j in 0..len1.min(128) {
                                        let b = self.maps.read_byte(buf1 + j).unwrap_or(0);
                                        if b == 0 {
                                            break;
                                        }
                                        s.push(if (0x20..=0x7e).contains(&b) {
                                            b as char
                                        } else {
                                            '?'
                                        });
                                    }
                                    log::trace!(
                                        "  [rdi.Buffer1=0x{:x}, len={}]: '{}'",
                                        buf1,
                                        len1,
                                        s
                                    );
                                    // Also try as wide string
                                    let mut sw = String::new();
                                    for j in 0..(len1 / 2).min(64) {
                                        let w = self.maps.read_word(buf1 + j * 2).unwrap_or(0);
                                        if w == 0 {
                                            break;
                                        }
                                        sw.push(char::from_u32(w as u32).unwrap_or('?'));
                                    }
                                    log::trace!("  [rdi.Buffer1 as wide]: '{}'", sw);
                                }
                                let buf2 = self.maps.read_qword(rdi + 0x18).unwrap_or(0);
                                let len2 = self.maps.read_word(rdi + 0x10).unwrap_or(0) as u64;
                                if buf2 != 0 && self.maps.is_mapped(buf2) {
                                    let mut s = String::new();
                                    for j in 0..len2.min(128) {
                                        let b = self.maps.read_byte(buf2 + j).unwrap_or(0);
                                        if b == 0 {
                                            break;
                                        }
                                        s.push(if (0x20..=0x7e).contains(&b) {
                                            b as char
                                        } else {
                                            '?'
                                        });
                                    }
                                    log::trace!(
                                        "  [rdi.Buffer2=0x{:x}, len={}]: '{}'",
                                        buf2,
                                        len2,
                                        s
                                    );
                                }
                                // Read 80 bytes at rsp+0x70 (the user32.rdata pointer)
                                let ptr = self.maps.read_qword(self.regs().rsp + 0x70).unwrap_or(0);
                                if ptr != 0 && self.maps.is_mapped(ptr) {
                                    let mut hex = String::new();
                                    let mut asc = String::new();
                                    for j in 0..80u64 {
                                        let b = self.maps.read_byte(ptr + j).unwrap_or(0);
                                        hex.push_str(&format!("{:02x} ", b));
                                        asc.push(if (0x20..=0x7e).contains(&b) {
                                            b as char
                                        } else {
                                            '.'
                                        });
                                    }
                                    log::trace!("  [rsp+0x70]@0x{:x} hex: {}", ptr, hex);
                                    log::trace!("  [rsp+0x70]@0x{:x} asc: {}", ptr, asc);
                                }
                                // Also check rsp+0x40, +0x50, +0x60 in case they hold the import name
                                for off in [0x20u64, 0x28, 0x30, 0x40, 0x48, 0x50, 0x58, 0x60, 0x68]
                                {
                                    let v =
                                        self.maps.read_qword(self.regs().rsp + off).unwrap_or(0);
                                    if v != 0 && self.maps.is_mapped(v) {
                                        // Try as ASCII string
                                        let mut s = String::new();
                                        for j in 0..64u64 {
                                            let b = self.maps.read_byte(v + j).unwrap_or(0);
                                            if b == 0 || !(0x20..=0x7e).contains(&b) {
                                                break;
                                            }
                                            s.push(b as char);
                                        }
                                        if s.len() >= 3 {
                                            log::trace!("  [rsp+0x{:x}]=0x{:x} → '{}'", off, v, s);
                                        }
                                    }
                                }
                            }
                        }
                        // ntdll+0x732b0 — unconditional `mov ebx, 0xc0000139` in some lookup function.
                        // Dump rcx, rdx (likely the DLL handle and the searched name).
                        0x18000732b0 => {
                            log::trace!(
                                "DEBUG ntdll+0x732b0 status=0xc0000139 setup rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x}",
                                self.regs().rcx,
                                self.regs().rdx,
                                self.regs().r8,
                                self.regs().r9
                            );
                            // rdx is often a PANSI_STRING for export name
                            if self.regs().rdx != 0 && self.maps.is_mapped(self.regs().rdx) {
                                // ANSI_STRING { WORD Length; WORD MaxLen; PCHAR Buffer (at +8) }
                                let len = self.maps.read_word(self.regs().rdx).unwrap_or(0);
                                let buf = self.maps.read_qword(self.regs().rdx + 8).unwrap_or(0);
                                if len > 0 && len < 512 && buf != 0 && self.maps.is_mapped(buf) {
                                    let mut s = String::new();
                                    for j in 0..(len as u64).min(256) {
                                        let b = self.maps.read_byte(buf + j).unwrap_or(0);
                                        if b == 0 {
                                            break;
                                        }
                                        s.push(b as char);
                                    }
                                    log::trace!("  rdx as ANSI_STRING (len={}): '{}'", len, s);
                                }
                            }
                            // Sometimes rcx is a pointer to the searched name
                            let mut s = String::new();
                            for j in 0..128 {
                                let b = self.maps.read_byte(self.regs().rcx + j).unwrap_or(0);
                                if b == 0 || !(0x20..=0x7e).contains(&b) {
                                    break;
                                }
                                s.push(b as char);
                            }
                            if !s.is_empty() {
                                log::trace!("  rcx as ASCII: '{}'", s);
                            }
                        }
                        // ntdll+0xb075c — LdrpSnapThunk: STATUS_ENTRYPOINT_NOT_FOUND path
                        // rdi = pointer to the unresolved name (likely an IMAGE_IMPORT_BY_NAME or similar)
                        // rbx = pointer to the DLL's LDR entry / structure
                        0x18000b075c => {
                            log::trace!(
                                "DEBUG LdrpSnapThunk ENTRYPOINT_NOT_FOUND: rdi=0x{:x} rbx=0x{:x} r13b=0",
                                self.regs().rdi,
                                self.regs().rbx
                            );
                            // IMAGE_IMPORT_BY_NAME = { WORD Hint; CHAR Name[1]; }
                            // The rdi at this point is offset 0x48 in some struct — read more around it
                            for off in [0u64, 8, 16, 24, 32, 40, 48] {
                                if self.maps.is_mapped(self.regs().rdi + off) {
                                    let v =
                                        self.maps.read_qword(self.regs().rdi + off).unwrap_or(0);
                                    log::trace!("  [rdi+0x{:x}] = 0x{:x}", off, v);
                                }
                            }
                            // Try reading rdi as ASCII (IMAGE_IMPORT_BY_NAME has Hint(2)+Name)
                            let mut s = String::new();
                            for j in 2..130 {
                                let b = self.maps.read_byte(self.regs().rdi + j).unwrap_or(0);
                                if b == 0 || !(0x20..=0x7e).contains(&b) {
                                    break;
                                }
                                s.push(b as char);
                            }
                            if !s.is_empty() {
                                log::trace!("  rdi+2 as ASCII: '{}'", s);
                            }
                        }
                        // ntdll+0xc3cb6 — error message prep for STATUS_ENTRYPOINT_NOT_FOUND
                        // Dump rax/rdx/rdi which usually carry pointers to the offending DLL/export.
                        0x1800c3cb6 => {
                            log::trace!(
                                "DEBUG ntdll+0xc3cb6 ENTRYPOINT_NOT_FOUND prep rax=0x{:x} rdx=0x{:x} rdi=0x{:x} r8=0x{:x} r9=0x{:x}",
                                self.regs().rax,
                                self.regs().rdx,
                                self.regs().rdi,
                                self.regs().r8,
                                self.regs().r9
                            );
                            // Try to read string at rdi (could be PUNICODE_STRING or PCSTR)
                            if self.regs().rdi != 0 && self.maps.is_mapped(self.regs().rdi) {
                                // First try as PUNICODE_STRING (Length WORD, MaxLen WORD, _, Buffer QWORD at +8)
                                let len = self.maps.read_word(self.regs().rdi).unwrap_or(0);
                                let buf = self.maps.read_qword(self.regs().rdi + 8).unwrap_or(0);
                                if len > 0 && len < 512 && buf != 0 && self.maps.is_mapped(buf) {
                                    let mut s = String::new();
                                    for j in 0..(len as u64 / 2).min(128) {
                                        let w = self.maps.read_word(buf + j * 2).unwrap_or(0);
                                        if w == 0 {
                                            break;
                                        }
                                        s.push(char::from_u32(w as u32).unwrap_or('?'));
                                    }
                                    log::trace!("  rdi as UNICODE_STRING: '{}'", s);
                                }
                                // Also try as raw ASCII / WCHAR
                                let ascii: String = (0..64)
                                    .filter_map(|j| {
                                        let b =
                                            self.maps.read_byte(self.regs().rdi + j).unwrap_or(0);
                                        if (0x20..=0x7e).contains(&b) {
                                            Some(b as char)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                if !ascii.is_empty() {
                                    log::trace!("  rdi as ASCII: '{}'", ascii);
                                }
                            }
                            // Same dump for rax which often points to the export-name UNICODE_STRING
                            if self.regs().rax != 0 && self.maps.is_mapped(self.regs().rax) {
                                let len = self.maps.read_word(self.regs().rax).unwrap_or(0);
                                let buf = self.maps.read_qword(self.regs().rax + 8).unwrap_or(0);
                                if len > 0 && len < 512 && buf != 0 && self.maps.is_mapped(buf) {
                                    let mut s = String::new();
                                    for j in 0..(len as u64 / 2).min(128) {
                                        let w = self.maps.read_word(buf + j * 2).unwrap_or(0);
                                        if w == 0 {
                                            break;
                                        }
                                        s.push(char::from_u32(w as u32).unwrap_or('?'));
                                    }
                                    log::trace!("  rax as UNICODE_STRING: '{}'", s);
                                }
                            }
                        }
                        // Catch the FIRST instruction that lives at the post-LdrInit
                        // memset hot spot. We hook a known-good ntdll address right
                        // after NtRaiseException returns so we can print enough state
                        // to identify the source of the 64 KB byte-wise write at
                        // 0x412000.
                        0x180103f7b => {
                            static SEEN: std::sync::atomic::AtomicUsize =
                                std::sync::atomic::AtomicUsize::new(0);
                            let n = SEEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if n == 0 {
                                let r = self.regs();
                                log::trace!(
                                    "DEBUG post-NtRaiseException at 0x180103f7b: rax=0x{:x} rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x} rsp=0x{:x} rbp=0x{:x}",
                                    r.rax,
                                    r.rcx,
                                    r.rdx,
                                    r.r8,
                                    r.r9,
                                    r.rsp,
                                    r.rbp,
                                );
                            }
                        }
                        // ntdll!RtlRaiseStatus entry (Win2022 build: RVA 0x106fd0).
                        // On real Windows an unhandled exception path through here
                        // terminates the process. In our emulator
                        // `RtlRaiseNoncontinuableException` returns instead of dying,
                        // which traps the function in a self-recursion that eats the
                        // entire stack. Bail cleanly on the first entry — equivalent
                        // to "unhandled exception → process terminated".
                        0x180106fd0 => {
                            let status = self.regs().get_ecx() as u32;
                            log::trace!(
                                "ntdll!RtlRaiseStatus(0x{:x}) at pos={} rsp=0x{:x} — terminating (no handler installed)",
                                status,
                                self.pos,
                                self.regs().rsp,
                            );
                            // Dump return-chain hint so it's clear who raised.
                            for i in 0..8u64 {
                                let a = self.regs().rsp.wrapping_add(i * 8);
                                if let Some(v) = self.maps.read_qword(a) {
                                    if (0x180000000..0x180400000).contains(&v)
                                        || (0x7ff000000000..0x7ff800000000).contains(&v)
                                    {
                                        log::trace!("  ret[+0x{:x}] = 0x{:x}", i * 8, v);
                                    }
                                }
                            }
                            self.process_terminated = true;
                            self.is_running
                                .store(0, std::sync::atomic::Ordering::Relaxed);
                            self.force_break = true;
                            return Ok(self.pc());
                        }
                        _ => {}
                    }

                    // --- Breakpoints ---
                    if (self.exp != u64::MAX && self.exp == self.pos)
                        || self.bp.is_bp_instruction(self.pos)
                        || self.bp.is_bp(addr)
                        || (self.cfg.console2 && self.cfg.console_addr == addr)
                    {
                        if self.running_script {
                            return Ok(self.pc());
                        }

                        self.cfg.console2 = false;
                        if self.cfg.verbose >= 2 {
                            let decoded = self.ensure_last_decoded_instruction(
                                &mut decoded,
                                is_aarch64,
                                addr,
                                x86_ins,
                                aarch64_ins,
                            );
                            let output = self.format_instruction(&decoded);
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
                    self.trace_pre_step_state(self.pos);

                    // --- Pre-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_pre_instruction.take() {
                        let decoded = self.ensure_last_decoded_instruction(
                            &mut decoded,
                            is_aarch64,
                            addr,
                            x86_ins,
                            aarch64_ins,
                        );
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
                    if !is_aarch64 && self.handle_x86_rep_pre_execution(x86_ins, sz) {
                        inner_running = self.instruction_cache_can_decode();
                        continue;
                    }

                    // --- Entropy ---
                    if self.cfg.entropy && self.pos % 10000 == 0 {
                        self.update_entropy();
                    }

                    // --- Verbose output ---
                    // Use `show_instruction` so the line gets the same color
                    // as the post-mortem dump and the x86 path; the previous
                    // raw `log::trace!` left aarch64 traces uncolored.
                    if self.cfg.verbose >= 2 && is_aarch64 {
                        let decoded = self.ensure_last_decoded_instruction(
                            &mut decoded,
                            is_aarch64,
                            addr,
                            x86_ins,
                            aarch64_ins,
                        );
                        self.show_instruction(color!("Cyan"), &decoded);
                    }

                    if !is_aarch64 {
                        win_syscall64_memory::ntdll_heap_list_walk_fixup(self, &x86_ins, addr);
                    }

                    // --- Emulate ---
                    let emulation_ok = if is_aarch64 {
                        engine::aarch64::emulate_instruction(self, &aarch64_ins)
                    } else {
                        engine::emulate_instruction(self, &x86_ins, sz, false)
                    };
                    self.last_instruction_size = sz;

                    if self.is_running.load(atomic::Ordering::Relaxed) == 0 {
                        return Ok(self.pc());
                    }

                    // --- x86 REP post-execution state machine ---
                    if !is_aarch64 {
                        self.update_x86_rep_state_after_execution(x86_ins);
                    }

                    // --- Post-instruction hook ---
                    if let Some(mut hook_fn) = self.hooks.hook_on_post_instruction.take() {
                        let decoded = self.ensure_last_decoded_instruction(
                            &mut decoded,
                            is_aarch64,
                            addr,
                            x86_ins,
                            aarch64_ins,
                        );
                        let hook_pc = self.pc();
                        hook_fn(self, hook_pc, &decoded, sz, emulation_ok);
                        self.hooks.hook_on_post_instruction = Some(hook_fn);
                    }

                    // --- Post-execution tracing ---
                    if self.cfg.inspect {
                        self.trace_memory_inspection();
                    }

                    if self.cfg.trace_regs
                        && self.cfg.trace_filename.is_some()
                        && self.pos >= self.cfg.trace_start
                    {
                        self.ensure_last_decoded_instruction(
                            &mut decoded,
                            is_aarch64,
                            addr,
                            x86_ins,
                            aarch64_ins,
                        );
                        self.capture_post_op();
                        self.write_to_trace_file();
                    }

                    // --- Register trace (aarch64) ---
                    if is_aarch64 && self.cfg.trace_regs {
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
                        break; // break inner loop to re-fetch from new PC
                    }

                    if is_aarch64 {
                        self.regs_aarch64_mut().pc += 4;
                    } else if self.rep.is_none() {
                        if self.cfg.is_x64() {
                            self.regs_mut().rip += sz as u64;
                        } else {
                            let new_eip = self.regs().get_eip() + sz as u64;
                            self.regs_mut().set_eip(new_eip);
                        }
                    }

                    if self.force_break {
                        self.force_break = false;
                        break;
                    }

                    // --- Return-based stop ---
                    // TODO: re-enable this. Correct semantics for `run_until_ret()` on
                    // BOTH arches (main's run_aarch64 has the equivalent check at
                    // execution_aarch64.rs:185). Currently disabled because main's x86
                    // path lacks this check entirely: instead, ret.rs returns true
                    // without updating rip when run_until_ret is set, the loop then
                    // advances rip += sz past the ret, execution falls through to
                    // whatever bytes follow, and eventually crashes into unmapped
                    // memory — at which point run() returns Err and callers using
                    // `let _ = emu.run_until_ret()` silently swallow it. The test
                    // tests::string_ops_tests::test_scasb relies on this quirk: its
                    // `jz +7` is intentionally aimed at a `ret` that's expected to
                    // act as a nop so execution falls through to `mov rbx, 1`. To
                    // turn this on, fix the test bytecode (jz offset 0x07 -> 0x08 so
                    // it skips both the `mov rbx, 0` AND its trailing ret, landing
                    // directly on `mov rbx, 1`), then uncomment the block below.
                    //
                    // if self.run_until_ret && decoded.is_return() {
                    //     return Ok(self.pc());
                    // }

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
    } // end run_single_threaded
}
