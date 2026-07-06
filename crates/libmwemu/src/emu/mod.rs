use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs::File,
    sync::{Arc, atomic::AtomicU32},
    time::Instant,
};

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

/// Architecture-specific instruction decoding and disassembly state.
/// Discriminated by target architecture so each variant carries only
/// the decode state relevant to its ISA.
pub enum ArchState {
    X86 {
        instruction: Option<iced_x86::Instruction>,
        formatter: iced_x86::IntelFormatter,
        instruction_cache: InstructionCache<iced_x86::Instruction>,
        decoder_position: usize,
    },
    AArch64 {
        instruction: Option<yaxpeax_arm::armv8::a64::Instruction>,
        instruction_cache: InstructionCache<yaxpeax_arm::armv8::a64::Instruction>,
    },
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
mod loaders;
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

pub struct Emu {
    // --- Configuration & display ---
    pub cfg: Config,
    pub colors: Colors,
    pub filename: String,

    // --- Memory & address space ---
    pub maps: Maps, // virtual memory map (all allocations, stack, heap, code regions)
    pub base: u64,  // base address for code loading
    pub heap_addr: u64, // current heap base address
    pub heap_management: Option<Box<O1Heap>>, // O(1) heap allocator for managed allocations
    pub memory_operations: Vec<MemoryOperation>, // per-step memory read/write log for tracing

    // --- Instruction decoding & disassembly ---
    pub arch_state: ArchState, // architecture-specific decode/cache/formatter state
    pub last_decoded: Option<DecodedInstruction>, // last decoded instruction when observers need
    // arch-neutral state; may be None on pure
    // execution fast paths.
    pub last_decoded_addr: u64, // address where `last_decoded` lived; needed
    // for state dumps because `pc()` already
    // reflects the *next* instruction (post-ret /
    // post-branch / post-advance) and would print
    // the wrong pc next to the last opcode.
    pub last_instruction_size: usize,
    pub rep: Option<u64>, // REP prefix counter for string operations

    // --- Core execution state ---
    pub pos: u64, // current instruction position counter (incremented each step)
    pub max_pos: Option<u64>, // optional execution position limit
    pub tick: usize, // global tick counter, used for thread scheduling
    pub is_running: Arc<AtomicU32>, // thread-safe flag for emulation running state
    pub ctrlc_console: Arc<AtomicU32>, // set by the Ctrl-C handler (--handle) to request dropping into the console at the next clean instruction boundary
    pub now: Instant,                  // timestamp of emulation start (wall-clock timing)
    pub force_break: bool, // set by breakpoints, memory violations, etc. to stop execution
    pub process_terminated: bool, // set by NtTerminateProcess; prevents run() from resetting is_running
    pub call_depth: u32, // nesting depth of call64/call32 — NtTerminateProcess only exits at depth 0
    pub ldr_init_done: bool, // true after LdrInitializeThunk call64 completes; switches API dispatch to virtual stubs
    pub force_reload: bool,  // trigger instruction re-decode
    pub run_until_ret: bool, // step-over mode: run until next RET
    pub rng: RefCell<rand::rngs::ThreadRng>,

    // --- Platform & loaded binary ---
    pub os: OperatingSystem,      // target OS (set by loader / init)
    pub pe64: Option<PE64>,       // parsed PE64 for runtime import resolution & resources
    pub pe32: Option<PE32>,       // parsed PE32 for runtime import resolution & resources
    // rs-header's PE parser is borrow-based (it does not keep the file bytes),
    // so libmwemu owns the raw image and passes it to rs-header methods that
    // need it (resource lookups, serialization re-parse). Never parsed here.
    pub pe64_raw: Option<Vec<u8>>,
    pub pe32_raw: Option<Vec<u8>>,
    pub elf64: Option<Elf64>,     // parsed ELF64 (Linux x86_64 / AArch64)
    pub elf32: Option<Elf32>,     // parsed ELF32 (Linux x86)
    pub macho64: Option<Macho64>, // parsed Mach-O 64 (macOS AArch64), includes addr_to_symbol
    pub tls_callbacks: Vec<u64>,  // PE TLS callback addresses
    pub library_loaded: bool,     // flag for GDB to detect library load events

    // --- Thread management ---
    pub threads: Vec<ThreadContext>,
    pub current_thread_id: usize,  // index into threads vec
    pub main_thread_cont: u64,     // main thread continuation/return address
    pub gateway_return: u64,       // return address from API gateway trampoline
    pub global_locks: GlobalLocks, // critical section/mutex tracking

    // --- API call interception ---
    pub hooks: Hooks,             // registered pre/post-instruction callback hooks
    pub skip_apicall: bool,       // stub/skip current API call
    pub its_apicall: Option<u64>, // address of API call currently being dispatched
    pub is_api_run: bool,         // true while inside a Windows/system API handler
    pub ld_bootstrap: bool, // Linux --libc: real ld.so is driving the bootstrap (no libc hooks)
    pub is_break_on_api: bool, // break on API calls (internal, for python interface)
    pub banzai: Banzai,     // auto-recovery: skip unimplemented APIs and continue

    // --- Debugging & breakpoints ---
    pub bp: Breakpoints, // address, instruction, and memory breakpoints
    pub break_on_alert: bool,
    pub break_on_next_cmp: bool,    // pause before next CMP instruction
    pub break_on_next_return: bool, // pause before next RET instruction
    pub enabled_ctrlc: bool,
    pub running_script: bool, // true while executing a debugger script
    pub exp: u64,             // instruction-count breakpoint: spawn console when pos == exp
    pub definitions: HashMap<u64, Definition>, // address annotations (duplicated from Config for serialization)
    pub stored_contexts: HashMap<String, StoredContext>, // named snapshots for breakpoint analysis

    // --- Tracing & statistics ---
    pub trace_file: Option<File>, // optional file handle for instruction trace output
    pub instruction_count: u64,   // total instructions executed
    pub fault_count: u32,         // page faults / exceptions encountered
    pub entropy: f64,             // entropy measurement for polymorphic code detection
    pub last_error: u32,          // Win32 GetLastError value

    // --- Win32 resource management ---
    pub handle_management: HandleManagement, // file and object handle table
    pub section_handles: HashMap<u64, String>, // KnownDll section handle → DLL filename (e.g., "kernel32.dll")
    pub file_handles: HashMap<u64, String>, // NtOpenFile handle → resolved basename (e.g., "kernelbase.dll"); used by NtCreateSection to inherit the dll name
    pub syscall_number_map: HashMap<u64, u64>, // real_nr (from loaded ntdll) → canonical_nr (the value our gateway dispatcher matches on). Built at init by scanning ntdll exports; empty means no translation.
    pub syscall_name_by_real: HashMap<u64, String>, // real_nr → "Nt<Name>" as exported by the loaded ntdll. Used in diagnostics so unimplemented-syscall logs name the right function (the static `what_syscall()` table is tied to a single Windows build and would otherwise mislabel cross-build syscalls).
    pub known_dll_dir_handles: HashSet<u64>, // handles returned by NtOpenDirectoryObject for \KnownDlls / \KnownDlls32; used by NtOpenSection to recognise relative DLL opens
    pub console_handles: HashSet<u64>, // handles backed by the console device (\Device\ConDrv\... and relative opens like \Reference / \Connect / \Input / \Output under a ConDrv root); used to recognise relative console opens and to answer NtDeviceIoControlFile on them
    pub api_resolve_cache: HashMap<String, u64>, // memoizes resolve_api_name_in_module: "module_lc\x01name" -> resolved VA. The resolver does an O(exports) string-read+lowercase scan per call; the loader resolves the same apiset imports ~100x (the kernelbase dance), so this dominated CPU without the cache. Only successful (non-zero) resolutions are cached.
    pub api_addr_name_cache: HashMap<u64, String>, // memoizes resolve_api_addr_to_name: VA -> export name. Same O(exports) scan; cached because module addresses are stable for a run.
    pub symbolic_link_targets: HashMap<u64, String>, // NtOpenSymbolicLinkObject handle → resolved link target (e.g. "\KnownDlls\KnownDllPath" → "C:\\Windows\\System32"); read back by NtQuerySymbolicLinkObject so ntdll's LdrInit can resolve the KnownDlls search path
    pub ssdt_pad_stack: Vec<u64>, // expected return addresses for PE→DLL CALLs that received an extra 0x20 of shadow-space padding (--ssdt only); a matching RET to PE pops and unpads
}

// --- ArchState accessors ---
impl Emu {
    /// Get the current x86 instruction (panics on aarch64).
    #[inline]
    pub fn x86_instruction(&self) -> Option<iced_x86::Instruction> {
        match &self.arch_state {
            ArchState::X86 { instruction, .. } => *instruction,
            ArchState::AArch64 { .. } => panic!("x86_instruction called on aarch64 emu"),
        }
    }

    /// Set the current x86 instruction.
    #[inline]
    pub fn set_x86_instruction(&mut self, ins: Option<iced_x86::Instruction>) {
        match &mut self.arch_state {
            ArchState::X86 { instruction, .. } => *instruction = ins,
            ArchState::AArch64 { .. } => panic!("set_x86_instruction called on aarch64 emu"),
        }
    }

    /// Get the x86 formatter (panics on aarch64).
    #[inline]
    pub fn x86_formatter(&mut self) -> &mut iced_x86::IntelFormatter {
        match &mut self.arch_state {
            ArchState::X86 { formatter, .. } => formatter,
            ArchState::AArch64 { .. } => panic!("x86_formatter called on aarch64 emu"),
        }
    }

    /// Get the x86 instruction cache (panics on aarch64).
    #[inline]
    pub fn x86_instruction_cache(&mut self) -> &mut InstructionCache<iced_x86::Instruction> {
        match &mut self.arch_state {
            ArchState::X86 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::AArch64 { .. } => panic!("x86_instruction_cache called on aarch64 emu"),
        }
    }

    /// Get the x86 instruction cache immutably.
    #[inline]
    pub fn x86_instruction_cache_ref(&self) -> &InstructionCache<iced_x86::Instruction> {
        match &self.arch_state {
            ArchState::X86 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::AArch64 { .. } => panic!("x86_instruction_cache_ref called on aarch64 emu"),
        }
    }

    /// Get the aarch64 instruction cache (panics on x86).
    #[inline]
    pub fn aarch64_instruction_cache(
        &mut self,
    ) -> &mut InstructionCache<yaxpeax_arm::armv8::a64::Instruction> {
        match &mut self.arch_state {
            ArchState::AArch64 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::X86 { .. } => panic!("aarch64_instruction_cache called on x86 emu"),
        }
    }

    /// Get the aarch64 instruction cache immutably.
    #[inline]
    pub fn aarch64_instruction_cache_ref(
        &self,
    ) -> &InstructionCache<yaxpeax_arm::armv8::a64::Instruction> {
        match &self.arch_state {
            ArchState::AArch64 {
                instruction_cache, ..
            } => instruction_cache,
            ArchState::X86 { .. } => panic!("aarch64_instruction_cache_ref called on x86 emu"),
        }
    }

    /// Get the x86 decoder position (panics on aarch64).
    #[inline]
    pub fn x86_decoder_position(&self) -> usize {
        match &self.arch_state {
            ArchState::X86 {
                decoder_position, ..
            } => *decoder_position,
            ArchState::AArch64 { .. } => panic!("x86_decoder_position called on aarch64 emu"),
        }
    }

    /// Set the x86 decoder position.
    #[inline]
    pub fn set_x86_decoder_position(&mut self, pos: usize) {
        match &mut self.arch_state {
            ArchState::X86 {
                decoder_position, ..
            } => *decoder_position = pos,
            ArchState::AArch64 { .. } => panic!("set_x86_decoder_position called on aarch64 emu"),
        }
    }

    /// Format an x86 instruction to a string using the Intel formatter.
    #[inline]
    pub fn x86_format_instruction(&mut self, ins: &iced_x86::Instruction) -> String {
        let mut output = String::new();
        match &mut self.arch_state {
            ArchState::X86 { formatter, .. } => {
                use iced_x86::Formatter as _;
                formatter.format(ins, &mut output);
            }
            ArchState::AArch64 { .. } => panic!("x86_format_instruction called on aarch64 emu"),
        }
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
}
