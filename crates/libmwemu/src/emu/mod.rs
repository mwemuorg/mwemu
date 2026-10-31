use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    sync::{Arc, atomic::AtomicU32},
    time::Instant,
};

use iced_x86::Formatter as _;

use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::disassemble::InstructionCache;
use crate::emu::object_handle::HandleManagement;
use crate::maps::heap_allocation::O1Heap;
use crate::{
    api::banzai::Banzai,
    arch::OperatingSystem,
    config::Config,
    debug::breakpoint::Breakpoints,
    debug::definitions::{Definition, StoredContext},
    hooks::Hooks,
    loaders::macho::macho64::Macho64,
    maps::Maps,
    threading::context::ThreadContext,
    threading::global_locks::GlobalLocks,
    utils::colors::Colors,
    windows::structures::MemoryOperation,
};
use rs_header::elf::{elf32::Elf32, elf64::Elf64};
use rs_header::pe::{pe32::PE32, pe64::PE64};

use crate::api::windows::export_index::ExportIndexRegistry;

/// One resolved call recorded while `cfg.trace_calls` is on (see
/// `engine/instructions/call.rs`). Only calls whose target resolves to a
/// name (a hooked winapi stub, or an export inside a real DLL loaded via
/// `--winver`/`--iso`/`--maps`) are kept — anonymous calls into the
/// binary's own code are skipped to keep the log small and useful.
#[derive(Debug, Clone)]
pub struct ApiCallLogEntry {
    pub pos: u64,  // instruction position counter at the CALL (Emu::pos)
    pub from: u64, // address of the CALL instruction
    pub to: u64,   // resolved call target
    pub name: String,
}

/// Cap on `Emu::api_call_log`: oldest entries are dropped once this is
/// reached, so a long-running trace can't grow the log unboundedly.
pub const API_CALL_LOG_CAP: usize = 20_000;

/// Architecture-neutral instruction decoding state for the active ISA.
/// The cache is `InstructionCache<DecodedInstruction>` because exactly one
/// target architecture is active per `Emu`. Insertion helpers
/// (`insert_x86_from_decoder` / `insert_aarch64_from_block`) wrap each
/// decoded value into the right `DecodedInstruction` variant.
pub(crate) struct InstructionState {
    pub(crate) instruction: Option<DecodedInstruction>,
    pub(crate) formatter: iced_x86::IntelFormatter,
    pub(crate) instruction_cache: InstructionCache<DecodedInstruction>,
}

impl Default for InstructionState {
    fn default() -> Self {
        let mut formatter = iced_x86::IntelFormatter::new();
        formatter.options_mut().set_digit_separator("");
        formatter.options_mut().set_first_operand_char_index(6);
        Self {
            instruction: None,
            formatter,
            instruction_cache: InstructionCache::new(),
        }
    }
}

mod banzai;
mod call_stack;
mod config;
mod console;
pub mod decoded_instruction;
pub mod disassemble;
mod display;
pub mod emu_context;
mod exception_handlers;
mod execution;
mod flags;
mod fls;
mod fpu;
mod fs;
mod initialization;
mod instruction_pointer;
mod iso;
pub(crate) mod loaders;
mod maps;
mod memory;
mod operands;
mod registers;
mod stack;
mod thread_context;
mod threading;
mod tls;
mod trace;
mod winapi;
pub mod winver;

pub mod object_handle;

/// The core emulator state machine. Holds CPU registers, memory maps,
/// configuration, and all runtime state for x86/x64/AArch64 emulation.
pub struct Emu {
    // --- Configuration & display ---
    pub cfg: Config,
    pub colors: Colors,
    pub filename: String,

    // --- Memory & address space ---
    pub maps: Maps, // virtual memory map (all allocations, stack, heap, code regions)
    pub base: u64,  // base address for code loading
    pub(crate) heap_addr: u64, // current heap base address
    #[allow(clippy::vec_box)] // O1Heap is large; boxing avoids costly moves on vec growth
    pub(crate) heap_arenas: Vec<Box<O1Heap>>, // index 0 = process heap; O(1) allocator per heap
    pub(crate) memory_operations: Vec<MemoryOperation>, // per-step memory read/write log for tracing

    pub(crate) instruction_state: InstructionState, // active ISA-specific decode/cache/formatter state
    pub last_decoded: Option<DecodedInstruction>,   // last decoded instruction when observers need
    // execution fast paths.
    pub(crate) last_decoded_addr: u64, // address where `last_decoded` lived; needed
    // for state dumps because `pc()` already
    // reflects the *next* instruction (post-ret /
    // post-branch / post-advance) and would print
    // the wrong pc next to the last opcode.
    pub last_instruction_size: usize,
    pub(crate) rep: Option<u64>, // REP prefix counter for string operations

    // --- Core execution state ---
    pub pos: u64, // current instruction position counter (incremented each step)
    pub(crate) max_pos: Option<u64>, // optional execution position limit
    pub(crate) tick: usize, // global tick counter, used for thread scheduling
    pub(crate) is_running: Arc<AtomicU32>, // thread-safe flag for emulation running state
    pub(crate) ctrlc_console: Arc<AtomicU32>, // set by the Ctrl-C handler (--handle) to request dropping into the console at the next clean instruction boundary
    pub(crate) now: Instant,                  // timestamp of emulation start (wall-clock timing)
    pub(crate) force_break: bool, // set by breakpoints, memory violations, etc. to stop execution
    pub process_terminated: bool, // set by NtTerminateProcess; prevents run() from resetting is_running
    pub(crate) call_depth: u32, // nesting depth of call64/call32 — NtTerminateProcess only exits at depth 0
    pub(crate) ldr_init_done: bool, // true after LdrInitializeThunk call64 completes; switches API dispatch to virtual stubs
    pub(crate) force_reload: bool,  // trigger instruction re-decode
    pub(crate) run_until_ret: bool, // step-over mode: run until next RET
    pub(crate) rng: rand::rngs::StdRng, // random source for rdrand/rdseed; reproducible via `set_seed`
    pub(crate) rng_seed: Option<u64>,   // seed `rng` was last seeded with (None = from the OS)

    // --- Platform & loaded binary ---
    pub os: OperatingSystem,       // target OS (set by loader / init)
    pub(crate) pe64: Option<PE64>, // parsed PE64 for runtime import resolution & resources
    pub(crate) pe32: Option<PE32>, // parsed PE32 for runtime import resolution & resources
    // rs-header's PE parser is borrow-based (it does not keep the file bytes),
    // so libmwemu owns the raw image and passes it to rs-header methods that
    // need it (resource lookups, serialization re-parse). Never parsed here.
    pub(crate) pe64_raw: Option<Vec<u8>>,
    pub(crate) pe32_raw: Option<Vec<u8>>,
    pub(crate) pe32_flat_loaded: bool, // true when PE32 was flat-loaded for a reflective loader
    pub(crate) elf64: Option<Elf64>,   // parsed ELF64 (Linux x86_64 / AArch64)
    pub(crate) elf32: Option<Elf32>,   // parsed ELF32 (Linux x86)
    pub(crate) macho64: Option<Macho64>, // parsed Mach-O 64 (macOS AArch64), includes addr_to_symbol
    // --- Kernel-mode (driver) emulation ---
    /// Present once a driver is loaded: the emulated kernel's address-space
    /// plan, API stubs, allocator ledger and memory-safety findings.
    pub kernel: Option<Box<crate::kernel::KernelEnv>>,
    /// Fast gate for the per-access lifetime checks. Only true while a driver
    /// is loaded, so the ordinary user-mode paths pay one predictable branch.
    pub(crate) kernel_guard: bool,
    pub(crate) tls_callbacks: Vec<u64>, // PE TLS callback addresses
    pub(crate) library_loaded: bool,    // flag for GDB to detect library load events

    // --- Thread management ---
    pub(crate) threads: Vec<ThreadContext>,
    pub(crate) current_thread_id: usize, // index into threads vec
    pub(crate) main_thread_cont: u64,    // main thread continuation/return address
    pub(crate) gateway_return: u64,      // return address from API gateway trampoline
    pub(crate) global_locks: GlobalLocks, // critical section/mutex tracking
    pub(crate) processes: crate::threading::process::ProcessTable, // Unix pid identity, fork() frames and unreaped children

    // --- API call interception ---
    pub hooks: Hooks,              // registered pre/post-instruction callback hooks
    pub skip_apicall: bool,        // stub/skip current API call
    pub its_apicall: Option<u64>,  // address of API call currently being dispatched
    pub(crate) is_api_run: bool,   // true while inside a Windows/system API handler
    pub(crate) ld_bootstrap: bool, // Linux --libc: real ld.so is driving the bootstrap (no libc hooks)
    pub is_break_on_api: bool,     // break on API calls (internal, for python interface)
    pub(crate) banzai: Banzai,     // auto-recovery: skip unimplemented APIs and continue

    // --- Debugging & breakpoints ---
    pub bp: Breakpoints, // address, instruction, and memory breakpoints
    pub(crate) break_on_alert: bool,
    pub(crate) break_on_next_cmp: bool, // pause before next CMP instruction
    pub(crate) break_on_next_return: bool, // pause before next RET instruction
    pub(crate) enabled_ctrlc: bool,
    pub running_script: bool, // true while executing a debugger script
    pub(crate) exp: u64,      // instruction-count breakpoint: spawn console when pos == exp
    pub(crate) definitions: HashMap<u64, Definition>, // address annotations (duplicated from Config for serialization)
    pub(crate) stored_contexts: HashMap<String, StoredContext>, // named snapshots for breakpoint analysis

    // --- Tracing & statistics ---
    pub(crate) trace_file: Option<File>, // optional file handle for instruction trace output
    pub api_call_log: VecDeque<ApiCallLogEntry>, // resolved calls seen while cfg.trace_calls is on (bounded, see API_CALL_LOG_CAP)
    pub instruction_count: u64,                  // total instructions executed
    pub fault_count: u32,                        // page faults / exceptions encountered
    pub(crate) entropy: f64, // entropy measurement for polymorphic code detection
    pub(crate) last_error: u64, // Win32 GetLastError value
    pub(crate) count_read: u32, // ReadFile call counter
    pub(crate) count_write: u32, // WriteFile call counter
    pub(crate) count_send: u32, // send/WSASend call counter
    pub(crate) count_recv: u32, // recv/WSARecv call counter

    // --- Win32 resource management ---
    pub(crate) handle_management: HandleManagement, // file and object handle table
    pub(crate) section_handles: HashMap<u64, String>, // KnownDll section handle → DLL filename (e.g., "kernel32.dll")
    pub(crate) file_handles: HashMap<u64, String>, // NtOpenFile handle → resolved basename (e.g., "kernelbase.dll"); used by NtCreateSection to inherit the dll name
    pub(crate) fts_handles: HashMap<u64, crate::api::macos::libsystem::FtsState>, // macOS FTS traversal state
    pub(crate) getopt_char_index: usize, // position within current argv element for grouped flags (-la)
    pub emulated_stdout: Vec<u8>,        // buffered stdout output from emulated program
    pub(crate) syscall_number_map: HashMap<u64, u64>, // real_nr (from loaded ntdll) → canonical_nr (the value our gateway dispatcher matches on). Built at init by scanning ntdll exports; empty means no translation.
    pub(crate) syscall_name_by_real: HashMap<u64, String>, // real_nr → "Nt<Name>" as exported by the loaded ntdll. Used in diagnostics so unimplemented-syscall logs name the right function (the static `what_syscall()` table is tied to a single Windows build and would otherwise mislabel cross-build syscalls).
    pub(crate) known_dll_dir_handles: HashSet<u64>, // handles returned by NtOpenDirectoryObject for \KnownDlls / \KnownDlls32; used by NtOpenSection to recognise relative DLL opens
    pub(crate) console_handles: HashSet<u64>, // handles backed by the console device (\Device\ConDrv\... and relative opens like \Reference / \Connect / \Input / \Output under a ConDrv root); used to recognise relative console opens and to answer NtDeviceIoControlFile on them
    pub(crate) api_resolve_cache: HashMap<String, u64>, // memoizes resolve_api_name_in_module: "module_lc\x01name" -> resolved VA. The resolver does an O(exports) string-read+lowercase scan per call; the loader resolves the same apiset imports ~100x (the kernelbase dance), so this dominated CPU without the cache. Only successful (non-zero) resolutions are cached.
    pub(crate) api_addr_name_cache: HashMap<u64, String>, // memoizes resolve_api_addr_to_name: VA -> export name. Same O(exports) scan; cached because module addresses are stable for a run.
    /// Persistent, host-side export-name index per mapped PE module. Built
    /// once at mapping/relocation time and used as the fast path for
    /// named/ordinal/address lookups. The PEB/export scanner remains as a
    /// compatibility fallback for unregistered or malformed modules.
    pub(crate) export_indexes: ExportIndexRegistry,
    pub(crate) symbolic_link_targets: HashMap<u64, String>, // NtOpenSymbolicLinkObject handle → resolved link target (e.g. "\KnownDlls\KnownDllPath" → "C:\\Windows\\System32"); read back by NtQuerySymbolicLinkObject so ntdll's LdrInit can resolve the KnownDlls search path
    pub(crate) ssdt_pad_stack: Vec<u64>, // expected return addresses for PE→DLL CALLs that received an extra 0x20 of shadow-space padding (--ssdt only); a matching RET to PE pops and unpads
}

// --- Random source ---
impl Emu {
    /// Re-seed the emulator's random source so `rdrand`/`rdseed` return the
    /// same sequence on every run. Also records the seed in `cfg.seed`.
    pub fn set_seed(&mut self, seed: u64) {
        use rand::SeedableRng as _;
        self.cfg.seed = Some(seed);
        self.rng_seed = Some(seed);
        self.rng = rand::rngs::StdRng::seed_from_u64(seed);
    }

    /// Apply `cfg.seed` if it changed since the random source was seeded.
    /// Called when a run starts, so `cfg.set_seed(..)` works without touching
    /// the emulator; `run_to` loops keep the sequence going.
    pub(crate) fn apply_cfg_seed(&mut self) {
        if let Some(seed) = self.cfg.seed
            && self.rng_seed != Some(seed)
        {
            self.set_seed(seed);
        }
    }

    /// Next 64-bit value from the emulator's random source.
    pub fn random_u64(&mut self) -> u64 {
        use rand::RngCore as _;
        self.rng.next_u64()
    }
}

// --- Win32 last-error helpers ---
impl Emu {
    /// Set the Win32 last-error code (GetLastError / SetLastError).
    pub fn set_last_error(&mut self, code: u64) {
        self.last_error = code;
    }

    /// Get the Win32 last-error code.
    pub fn get_last_error(&self) -> u64 {
        self.last_error
    }

    /// Reset the Win32 last-error code to zero.
    pub fn clear_last_error(&mut self) {
        self.last_error = 0;
    }
}

// --- InstructionState accessors ---
impl Emu {
    /// Get the current x86 instruction (panics on aarch64).
    #[inline]
    pub fn x86_instruction(&self) -> Option<iced_x86::Instruction> {
        match &self.instruction_state.instruction {
            Some(DecodedInstruction::X86(ins)) => Some(*ins),
            Some(DecodedInstruction::AArch64(_)) => {
                unreachable!("x86_instruction called on aarch64 emu")
            }
            None => None,
        }
    }

    /// Set the current x86 instruction.
    #[inline]
    pub fn set_x86_instruction(&mut self, ins: Option<iced_x86::Instruction>) {
        self.instruction_state.instruction = ins.map(DecodedInstruction::X86);
    }

    /// Get the current aarch64 instruction (panics on x86).
    #[inline]
    pub fn aarch64_instruction(&self) -> Option<yaxpeax_arm::armv8::a64::Instruction> {
        match &self.instruction_state.instruction {
            Some(DecodedInstruction::AArch64(ins)) => Some(*ins),
            Some(DecodedInstruction::X86(_)) => {
                unreachable!("aarch64_instruction called on x86 emu")
            }
            None => None,
        }
    }

    /// Set the current aarch64 instruction.
    #[inline]
    pub fn set_aarch64_instruction(&mut self, ins: Option<yaxpeax_arm::armv8::a64::Instruction>) {
        self.instruction_state.instruction = ins.map(DecodedInstruction::AArch64);
    }

    /// Get the x86 formatter (panics on aarch64).
    #[inline]
    pub fn x86_formatter(&mut self) -> &mut iced_x86::IntelFormatter {
        self.assert_x86_inline("x86_formatter");
        &mut self.instruction_state.formatter
    }

    /// Get the active instruction cache (architecture-neutral).
    #[inline]
    pub fn instruction_cache(&mut self) -> &mut InstructionCache<DecodedInstruction> {
        &mut self.instruction_state.instruction_cache
    }

    /// Get the active instruction cache immutably (architecture-neutral).
    #[inline]
    pub fn instruction_cache_ref(&self) -> &InstructionCache<DecodedInstruction> {
        &self.instruction_state.instruction_cache
    }

    /// Get the x86 instruction cache (panics on aarch64).
    #[inline]
    pub fn x86_instruction_cache(&mut self) -> &mut InstructionCache<DecodedInstruction> {
        &mut self.instruction_state.instruction_cache
    }

    /// Get the x86 instruction cache immutably.
    #[inline]
    pub fn x86_instruction_cache_ref(&self) -> &InstructionCache<DecodedInstruction> {
        &self.instruction_state.instruction_cache
    }

    /// Get the aarch64 instruction cache (panics on x86).
    #[inline]
    pub fn aarch64_instruction_cache(&mut self) -> &mut InstructionCache<DecodedInstruction> {
        self.assert_aarch64_inline("aarch64_instruction_cache");
        &mut self.instruction_state.instruction_cache
    }

    /// Get the aarch64 instruction cache immutably.
    #[inline]
    pub fn aarch64_instruction_cache_ref(&self) -> &InstructionCache<DecodedInstruction> {
        self.assert_aarch64_inline("aarch64_instruction_cache_ref");
        &self.instruction_state.instruction_cache
    }

    /// Format an x86 instruction to a string using the Intel formatter.
    #[inline]
    pub fn x86_format_instruction(&mut self, ins: &iced_x86::Instruction) -> String {
        self.assert_x86_inline("x86_format_instruction");
        let mut output = String::new();
        use iced_x86::Formatter as _;
        self.instruction_state.formatter.format(ins, &mut output);
        output
    }

    /// Format a `DecodedInstruction` to a human-readable string.
    ///
    /// Dispatches to `IntelFormatter` for x86 or `Display` for aarch64.
    #[inline]
    pub fn format_instruction(&mut self, ins: &DecodedInstruction) -> String {
        match ins {
            DecodedInstruction::X86(x86_ins) => self.x86_format_instruction(x86_ins),
            DecodedInstruction::AArch64(aarch64_ins) => format!("{}", aarch64_ins),
        }
    }

    #[inline(always)]
    fn assert_x86_inline(&self, method: &'static str) {
        if !self.cfg.arch.is_x86() {
            panic!(
                "{} called on non-x86 emulator (arch={:?})",
                method, self.cfg.arch
            );
        }
    }

    #[inline(always)]
    fn assert_aarch64_inline(&self, method: &'static str) {
        if !self.cfg.arch.is_aarch64() {
            panic!(
                "{} called on non-AArch64 emulator (arch={:?})",
                method, self.cfg.arch
            );
        }
    }
}
