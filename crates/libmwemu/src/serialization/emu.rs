use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic;

use iced_x86::Instruction;
use serde::{Deserialize, Serialize};

use crate::api::banzai::Banzai;
use crate::config::Config;
use crate::debug::breakpoint::Breakpoints;
use crate::eflags::Eflags;
use crate::emu::Emu;
use crate::flags::Flags;
use crate::hooks::Hooks;
use crate::regs_aarch64::RegsAarch64;
use crate::regs64::Regs64;
use crate::serialization::fpu::SerializableFPU;
use crate::serialization::instant::SerializableInstant;
use crate::serialization::maps::SerializableMaps;
use crate::serialization::pe32::SerializablePE32;
use crate::serialization::pe64::SerializablePE64;
use crate::serialization::thread_context::SerializableThreadContext;
use crate::threading::global_locks::GlobalLocks;
use crate::utils::colors::Colors;
use crate::windows::structures::MemoryOperation;

use crate::arch::Arch;
use crate::emu::ArchState;
use crate::emu::disassemble::InstructionCache;
use crate::emu::object_handle::HandleManagement;
use crate::threading::context::ArchThreadState;

#[derive(Serialize, Deserialize)]
pub enum SerializableInstructionState {
    X86 {
        instruction: Option<Instruction>,
        decoder_position: usize,
    },
    AArch64,
}

#[derive(Serialize, Deserialize)]
pub enum SerializableCurrentThreadState {
    X86 {
        regs: Regs64,
        pre_op_regs: Regs64,
        post_op_regs: Regs64,
        flags: Flags,
        pre_op_flags: Flags,
        post_op_flags: Flags,
        eflags: Eflags,
        fpu: SerializableFPU,
        seh: u64,
        veh: u64,
        uef: u64,
        eh_ctx: u64,
        tls32: Vec<u32>,
        tls64: Vec<u64>,
        fls: Vec<u32>,
        fs: BTreeMap<u64, u64>,
        call_stack: Vec<(u64, u64)>,
    },
    AArch64 {
        regs: RegsAarch64,
        pre_op_regs: RegsAarch64,
        post_op_regs: RegsAarch64,
    },
}

#[derive(Serialize, Deserialize)]
pub struct SerializableEmu {
    // --- Configuration & display ---
    pub cfg: Config,
    pub colors: Colors,
    pub filename: String,

    // --- Memory & address space ---
    pub maps: SerializableMaps,
    pub base: u64,
    pub heap_addr: u64,
    // NOTE: heap_management is not serialized (runtime only)
    pub memory_operations: Vec<MemoryOperation>,

    // --- Instruction decoding & disassembly ---
    // NOTE: formatter, instruction_cache are recreated on deserialize
    pub instruction_state: SerializableInstructionState,
    pub last_instruction_size: usize,
    pub rep: Option<u64>,

    // --- Core execution state ---
    pub pos: u64,
    pub max_pos: Option<u64>,
    pub tick: usize,
    pub is_running: u32, // serialized as plain u32 (not atomic)
    pub now: SerializableInstant,
    pub force_break: bool,
    pub force_reload: bool,
    pub run_until_ret: bool,
    // NOTE: rng is not serialized (recreated on deserialize)

    // --- Platform & loaded binary ---
    pub os: crate::arch::OperatingSystem,
    pub pe64: Option<SerializablePE64>,
    pub pe32: Option<SerializablePE32>,
    // NOTE: elf32, elf64, macho64 not yet serialized
    pub tls_callbacks: Vec<u64>,

    // --- Thread management ---
    pub threads: Vec<SerializableThreadContext>,
    pub current_thread_id: usize,
    pub main_thread_cont: u64,
    pub gateway_return: u64,
    // NOTE: global_locks reset on deserialize

    // --- Flattened thread context (from current thread at serialize time) ---
    pub current_thread: SerializableCurrentThreadState,

    // --- API call interception ---
    // NOTE: hooks cannot be serialized
    pub banzai: Banzai,
    pub skip_apicall: bool,
    pub its_apicall: Option<u64>,
    // NOTE: is_api_run, is_break_on_api are runtime-only

    // --- Debugging & breakpoints ---
    pub bp: Breakpoints,
    pub break_on_alert: bool,
    pub break_on_next_cmp: bool,
    pub break_on_next_return: bool,
    pub enabled_ctrlc: bool,
    pub running_script: bool,
    pub exp: u64,
    // NOTE: definitions, stored_contexts reset on deserialize

    // --- Tracing & statistics ---
    // NOTE: trace_file reconstructed from cfg.trace_filename
    pub entropy: f64,
    pub last_error: u32,
    // NOTE: instruction_count, fault_count not serialized
    // NOTE: handle_management not yet serialized
}

impl<'a> From<&'a Emu> for SerializableEmu {
    fn from(emu: &'a Emu) -> Self {
        let instruction_state = match &emu.arch_state {
            ArchState::X86 {
                instruction,
                decoder_position,
                ..
            } => SerializableInstructionState::X86 {
                instruction: *instruction,
                decoder_position: *decoder_position,
            },
            ArchState::AArch64 { .. } => SerializableInstructionState::AArch64,
        };

        let current_thread = match &emu.current_thread().arch {
            ArchThreadState::X86 {
                regs,
                pre_op_regs,
                post_op_regs,
                flags,
                pre_op_flags,
                post_op_flags,
                eflags,
                fpu,
                seh,
                veh,
                uef,
                eh_ctx,
                tls32,
                tls64,
                fls,
                fs,
                call_stack,
            } => SerializableCurrentThreadState::X86 {
                regs: *regs,
                pre_op_regs: *pre_op_regs,
                post_op_regs: *post_op_regs,
                flags: *flags,
                pre_op_flags: *pre_op_flags,
                post_op_flags: *post_op_flags,
                eflags: eflags.clone(),
                fpu: fpu.clone().into(),
                seh: *seh,
                veh: *veh,
                uef: *uef,
                eh_ctx: *eh_ctx,
                tls32: tls32.clone(),
                tls64: tls64.clone(),
                fls: fls.clone(),
                fs: fs.clone(),
                call_stack: call_stack.clone(),
            },
            ArchThreadState::AArch64 {
                regs,
                pre_op_regs,
                post_op_regs,
            } => SerializableCurrentThreadState::AArch64 {
                regs: *regs,
                pre_op_regs: *pre_op_regs,
                post_op_regs: *post_op_regs,
            },
        };

        SerializableEmu {
            // Configuration & display
            cfg: emu.cfg.clone(),
            colors: emu.colors.clone(),
            filename: emu.filename.clone(),
            // Memory & address space
            maps: emu.maps.clone().into(),
            base: emu.base,
            heap_addr: emu.heap_addr,
            memory_operations: emu.memory_operations.clone(),
            // Instruction decoding
            instruction_state,
            last_instruction_size: emu.last_instruction_size,
            rep: emu.rep,
            // Core execution state
            pos: emu.pos,
            max_pos: emu.max_pos,
            tick: emu.tick,
            is_running: emu.is_running.load(std::sync::atomic::Ordering::Relaxed),
            now: SerializableInstant::from(emu.now),
            force_break: emu.force_break,
            force_reload: emu.force_reload,
            run_until_ret: emu.run_until_ret,
            // Platform & loaded binary
            os: emu.os,
            // rs-header's PE is borrow-based, so the raw bytes live in
            // `emu.pe{32,64}_raw`; pair them with the parsed filename.
            pe64: emu.pe64.as_ref().map(|pe| SerializablePE64 {
                filename: pe.filename.clone(),
                raw: emu.pe64_raw.clone().unwrap_or_default(),
            }),
            pe32: emu.pe32.as_ref().map(|pe| SerializablePE32 {
                filename: pe.filename.clone(),
                raw: emu.pe32_raw.clone().unwrap_or_default(),
            }),
            tls_callbacks: emu.tls_callbacks.clone(),
            // Thread management
            threads: emu.threads.iter().map(|t| t.into()).collect(),
            current_thread_id: emu.current_thread_id,
            main_thread_cont: emu.main_thread_cont,
            gateway_return: emu.gateway_return,
            // Flattened thread context
            current_thread,
            // API call interception
            banzai: emu.banzai.clone(),
            skip_apicall: emu.skip_apicall,
            its_apicall: emu.its_apicall,
            // Debugging & breakpoints
            bp: emu.bp.clone(),
            break_on_alert: emu.break_on_alert,
            break_on_next_cmp: emu.break_on_next_cmp,
            break_on_next_return: emu.break_on_next_return,
            enabled_ctrlc: emu.enabled_ctrlc,
            running_script: emu.running_script,
            exp: emu.exp,
            // Tracing & statistics
            entropy: emu.entropy,
            last_error: emu.last_error,
        }
    }
}

impl From<SerializableEmu> for Emu {
    fn from(serialized: SerializableEmu) -> Self {
        let SerializableEmu {
            cfg,
            colors,
            filename,
            maps,
            base,
            heap_addr,
            memory_operations,
            instruction_state,
            last_instruction_size,
            rep,
            pos,
            max_pos,
            tick,
            is_running,
            now,
            force_break,
            force_reload,
            run_until_ret,
            os,
            pe64,
            pe32,
            tls_callbacks,
            threads,
            current_thread_id,
            main_thread_cont,
            gateway_return,
            current_thread,
            banzai,
            skip_apicall,
            its_apicall,
            bp,
            break_on_alert,
            break_on_next_cmp,
            break_on_next_return,
            enabled_ctrlc,
            running_script,
            exp,
            entropy,
            last_error,
        } = serialized;

        let trace_file = if let Some(trace_filename) = &cfg.trace_filename {
            let file = File::open(trace_filename.clone()).unwrap();
            Some(file)
        } else {
            None
        };

        let arch_state = match instruction_state {
            SerializableInstructionState::X86 {
                instruction,
                decoder_position,
            } => ArchState::X86 {
                instruction,
                formatter: Default::default(),
                instruction_cache: InstructionCache::new(),
                decoder_position,
            },
            SerializableInstructionState::AArch64 => ArchState::AArch64 {
                instruction: None,
                instruction_cache: InstructionCache::new(),
            },
        };

        // rs-header's PE is borrow-based; keep the raw bytes alongside the
        // re-parsed PE so runtime resource lookups still work after restore.
        let pe64_raw = pe64.as_ref().map(|x| x.raw.clone());
        let pe32_raw = pe32.as_ref().map(|x| x.raw.clone());

        let mut emu = Emu {
            // Configuration & display
            cfg: cfg.clone(),
            colors,
            filename,
            // Memory & address space
            maps: maps.into(),
            base,
            heap_addr,
            heap_management: None,
            memory_operations,
            // Instruction decoding (formatter, cache recreated)
            arch_state,
            last_decoded: None,
            last_decoded_addr: 0,
            last_instruction_size,
            rep,
            // Core execution state
            pos,
            max_pos,
            tick,
            is_running: Arc::new(atomic::AtomicU32::new(is_running)),
            ctrlc_console: Arc::new(atomic::AtomicU32::new(0)),
            now: now.to_instant(),
            force_break,
            process_terminated: false,
            call_depth: 0,
            ldr_init_done: false,
            force_reload,
            run_until_ret,
            rng: RefCell::new(rand::rng()),
            // Platform & loaded binary
            os,
            pe64: pe64.map(|x| x.into()),
            pe32: pe32.map(|x| x.into()),
            pe64_raw,
            pe32_raw,
            elf64: None,    // TODO: not yet serialized
            elf32: None,    // TODO: not yet serialized
            macho64: None,  // TODO: not yet serialized
            tls_callbacks,
            library_loaded: false,
            // Thread management
            threads: threads.into_iter().map(|t| t.into()).collect(),
            current_thread_id,
            main_thread_cont,
            gateway_return,
            global_locks: GlobalLocks::new(),
            // API call interception (hooks cannot be serialized)
            hooks: Hooks::default(),
            skip_apicall,
            its_apicall,
            is_api_run: false,
            ld_bootstrap: false,
            is_break_on_api: false,
            banzai,
            // Debugging & breakpoints
            bp,
            break_on_alert,
            break_on_next_cmp,
            break_on_next_return,
            enabled_ctrlc,
            running_script,
            exp,
            definitions: HashMap::new(),
            stored_contexts: HashMap::new(),
            // Tracing & statistics
            trace_file,
            instruction_count: 0,
            fault_count: 0,
            entropy,
            last_error,
            // Win32 resource management
            handle_management: HandleManagement::new(), // TODO: not yet serialized
            section_handles: HashMap::new(),
            file_handles: HashMap::new(),
            syscall_number_map: HashMap::new(),
            syscall_name_by_real: HashMap::new(),
            known_dll_dir_handles: HashSet::new(),
            console_handles: HashSet::new(),
            api_resolve_cache: HashMap::new(),
            api_addr_name_cache: HashMap::new(),
            symbolic_link_targets: HashMap::new(),
            ssdt_pad_stack: Vec::new(),
        };

        if let Some(thread) = emu.threads.get_mut(current_thread_id) {
            match current_thread {
                SerializableCurrentThreadState::X86 {
                    regs,
                    pre_op_regs,
                    post_op_regs,
                    flags,
                    pre_op_flags,
                    post_op_flags,
                    eflags,
                    fpu,
                    seh,
                    veh,
                    uef,
                    eh_ctx,
                    tls32,
                    tls64,
                    fls,
                    fs,
                    call_stack,
                } => match &mut thread.arch {
                    ArchThreadState::X86 {
                        regs: thread_regs,
                        pre_op_regs: thread_pre_op_regs,
                        post_op_regs: thread_post_op_regs,
                        flags: thread_flags,
                        pre_op_flags: thread_pre_op_flags,
                        post_op_flags: thread_post_op_flags,
                        eflags: thread_eflags,
                        fpu: thread_fpu,
                        seh: thread_seh,
                        veh: thread_veh,
                        uef: thread_uef,
                        eh_ctx: thread_eh_ctx,
                        tls32: thread_tls32,
                        tls64: thread_tls64,
                        fls: thread_fls,
                        fs: thread_fs,
                        call_stack: thread_call_stack,
                    } => {
                        *thread_regs = regs;
                        *thread_pre_op_regs = pre_op_regs;
                        *thread_post_op_regs = post_op_regs;
                        *thread_flags = flags;
                        *thread_pre_op_flags = pre_op_flags;
                        *thread_post_op_flags = post_op_flags;
                        *thread_eflags = eflags;
                        *thread_fpu = fpu.into();
                        *thread_seh = seh;
                        *thread_veh = veh;
                        *thread_uef = uef;
                        *thread_eh_ctx = eh_ctx;
                        *thread_tls32 = tls32;
                        *thread_tls64 = tls64;
                        *thread_fls = fls;
                        *thread_fs = fs;
                        *thread_call_stack = call_stack;
                    }
                    ArchThreadState::AArch64 { .. } => {
                        thread.arch = ArchThreadState::X86 {
                            regs,
                            pre_op_regs,
                            post_op_regs,
                            flags,
                            pre_op_flags,
                            post_op_flags,
                            eflags,
                            fpu: fpu.into(),
                            seh,
                            veh,
                            uef,
                            eh_ctx,
                            tls32,
                            tls64,
                            fls,
                            fs,
                            call_stack,
                        };
                    }
                },
                SerializableCurrentThreadState::AArch64 {
                    regs,
                    pre_op_regs,
                    post_op_regs,
                } => match &mut thread.arch {
                    ArchThreadState::AArch64 {
                        regs: thread_regs,
                        pre_op_regs: thread_pre_op_regs,
                        post_op_regs: thread_post_op_regs,
                    } => {
                        *thread_regs = regs;
                        *thread_pre_op_regs = pre_op_regs;
                        *thread_post_op_regs = post_op_regs;
                    }
                    ArchThreadState::X86 { .. } => {
                        thread.arch = ArchThreadState::AArch64 {
                            regs,
                            pre_op_regs,
                            post_op_regs,
                        };
                    }
                },
            }
        }

        if emu.cfg.arch.is_64bits() {
            emu.maps.is_64bits = true;
        }

        emu
    }
}

impl Default for SerializableEmu {
    fn default() -> Self {
        SerializableEmu::from(&Emu::new(crate::arch::Arch::X86))
    }
}

impl SerializableEmu {
    pub fn set_regs(&mut self, regs: Regs64) {
        match &mut self.current_thread {
            SerializableCurrentThreadState::X86 {
                regs: current_regs, ..
            } => *current_regs = regs,
            SerializableCurrentThreadState::AArch64 { .. } => {
                panic!("set_regs called on aarch64 serialized thread state")
            }
        }
    }

    pub fn set_flags(&mut self, flags: Flags) {
        match &mut self.current_thread {
            SerializableCurrentThreadState::X86 {
                flags: current_flags,
                pre_op_flags,
                post_op_flags,
                ..
            } => {
                *current_flags = flags;
                *pre_op_flags = flags;
                *post_op_flags = flags;
            }
            SerializableCurrentThreadState::AArch64 { .. } => {
                panic!("set_flags called on aarch64 serialized thread state")
            }
        }
    }

    pub fn set_maps(&mut self, maps: SerializableMaps) {
        self.maps = maps;
    }

    pub fn set_pe32(&mut self, pe32: Option<SerializablePE32>) {
        self.pe32 = pe32;
    }

    pub fn set_pe64(&mut self, pe64: Option<SerializablePE64>) {
        self.pe64 = pe64;
    }

    pub fn set_regs_aarch64(&mut self, regs: RegsAarch64) {
        match &mut self.current_thread {
            SerializableCurrentThreadState::AArch64 {
                regs: current_regs,
                pre_op_regs,
                post_op_regs,
            } => {
                *current_regs = regs;
                *pre_op_regs = regs;
                *post_op_regs = regs;
            }
            SerializableCurrentThreadState::X86 { .. } => {
                panic!("set_regs_aarch64 called on x86 serialized thread state")
            }
        }
    }

    pub fn default_for_arch(arch: Arch) -> Self {
        let mut emu = match arch {
            Arch::Aarch64 => crate::emu_aarch64(),
            Arch::X86_64 => crate::emu64(),
            Arch::X86 => crate::emu32(),
        };
        emu.init_cpu();
        SerializableEmu::from(&emu)
    }
}
