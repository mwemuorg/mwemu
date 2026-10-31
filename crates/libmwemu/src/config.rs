use std::collections::HashMap;

use crate::arch::Arch;
use crate::{debug::definitions::Definition, windows::constants};
use serde::{Deserialize, Serialize};

/// Emulation configuration controlling architecture, memory layout, tracing, and runtime limits.
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    // --- Binary & architecture ---
    pub filename: String, // filename with full path included
    pub arch: Arch,       // CPU architecture (X86, X86_64, Aarch64)
    pub shellcode: bool,
    pub arguments: String,

    // --- Memory layout ---
    pub code_base_addr: u64,
    pub stack_addr: u64,
    pub entry_point: u64,
    pub maps_folder: String,
    pub winver: Option<String>, // --winver: resolved Windows build; system DLLs are fetched on demand from the symbol server

    // --- Heap behavior ---
    pub max_alloc_size: u64, // allocation cap (default 16MB), larger are truncated
    pub heap_alloc_min_size: u64, // minimum padding for all heap allocations

    // --- Tracing ---
    pub trace_mem: bool,   // show memory operations in every step
    pub trace_calls: bool, // trace every call
    pub trace_regs: bool,  // show all regs in every step
    pub trace_reg: bool,   // show value and content of a reg in every step
    pub trace_string: bool,
    pub trace_flags: bool,
    pub trace_filename: Option<String>,
    pub trace_start: u64,
    pub reg_names: Vec<String>, // which reg to trace
    pub stack_trace: bool,

    // --- Verbosity & output ---
    pub verbose: u32, // 0 = api only, 1 = api + messages, 2 = asm code
    pub verbose_at: Option<u64>,
    pub nocolors: bool, // disable colors for file redirection

    // --- Console / interactive debugger ---
    // Three trigger mechanisms: by instruction count, by address, or always-on
    pub console: bool,         // trigger console at instruction count `console_num`
    pub console_num: u64,      // instruction count at which to spawn console (sets emu.exp)
    pub console2: bool,        // trigger console when execution hits `console_addr` (one-shot)
    pub console_addr: u64,     // address to trigger console spawn (paired with console2)
    pub console_enabled: bool, // master gate for all console operations
    pub command: Option<String>,

    // --- Execution limits ---
    pub exit_position: u64,
    pub max_instructions: Option<u64>, // stop after N instructions
    pub timeout_secs: Option<f64>,     // stop after N seconds wall-clock time
    pub max_faults: Option<u32>,       // stop after N faults/exceptions
    pub seed: Option<u64>, // seed for the emulator's random source (rdrand/rdseed); None = seeded from the OS
    pub loops: bool,       // count iterations per instruction (slow)
    pub allow_empty_code_blocks: bool, // disable empty-code-block detector

    // --- API emulation behavior ---
    pub emulate_winapi: bool,
    pub linux_real_libc: bool, // execute the real mapped libc/ld code, only intercept `syscall`
    pub skip_unimplemented: bool,
    pub short_circuit_sleep: bool, // Sleep/Wait calls advance tick but return immediately

    // --- Threading ---
    pub enable_threading: bool,

    // --- Memory inspection ---
    pub inspect: bool,       // enable memory watch during execution
    pub inspect_seq: String, // operand to watch, e.g. "dword ptr [ebp + 0x24]"

    // --- Network emulation ---
    pub endpoint: bool, // enable network endpoint emulation (sockets, HTTP)

    // --- API emulation helpers ---
    pub string_addr: u64, // buffer address for string-returning API stubs

    // --- Analysis ---
    pub entropy: bool, // enable entropy measurement (polymorphic code detection)
    pub memory_guard: bool, // enable heap memory-safety analysis (UAF, double-free, overflow, leaks)
    pub definitions: HashMap<u64, Definition>, // address annotations (also duplicated on Emu for serialization)

    // --- Verbose range ---
    pub verbose_start: u64, // enable verbose when pos >= verbose_start (0 = disabled)
    pub verbose_end: u64,   // disable verbose when pos > verbose_end (0 = no end)
    pub verbose_range_saved: Option<u32>, // saved verbose level while inside the range

    // --- Dump on exit ---
    pub dump_on_exit: bool,
    pub dump_filename: Option<String>,

    // --- Simulated environment constants ---
    pub module_name: String,
    pub exe_name: String,
    pub user_name: String,
    pub host_name: String,
    pub temp_path: String,
    pub cwd_path: String,
    pub windows_directory: String,
    pub system_directory: String,

    // --- SSDT / loader bootstrap ---
    pub ssdt_use_ldr_initialize_thunk: bool, // use ntdll!LdrInitializeThunk for PE64 loader init

    // --- Fault tolerance ---
    pub soft_exception_recovery: bool, // continue past unhandled memory faults instead of terminating

    // --- Testing ---
    pub test_mode: bool,
    pub emulate_winapi_once: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Create a new Config with sensible defaults (x86, no tracing, no console).
    pub fn new() -> Config {
        Config {
            filename: String::new(),
            trace_mem: false,
            trace_calls: false,
            trace_regs: false,
            trace_reg: false,
            trace_filename: None,
            trace_start: 0,
            trace_string: false,
            trace_flags: false,
            reg_names: Vec::new(),
            verbose: 0,
            console: false,
            console_num: 0,
            loops: false,
            nocolors: false,
            string_addr: 0,
            inspect: false,
            inspect_seq: "".to_string(),
            endpoint: false,
            maps_folder: "".to_string(),
            winver: None,
            console2: false,
            console_addr: 0,
            entry_point: constants::CFG_DEFAULT_BASE,
            exit_position: 0,
            dump_on_exit: false, // TODO: a way to make it false/set it through cli + lib
            dump_filename: Some("dumps/emu.bin".to_string()), // TODO: a way to set it through cli + lib
            code_base_addr: constants::CFG_DEFAULT_BASE,
            arch: Arch::X86,
            stack_trace: false,
            test_mode: false,
            console_enabled: false,
            skip_unimplemented: false,
            stack_addr: 0,
            arguments: "".to_string(),
            enable_threading: false, // Default to single-threaded for backward compatibility
            verbose_at: None,
            verbose_start: 0,
            verbose_end: 0,
            verbose_range_saved: None,
            command: None,
            definitions: HashMap::new(),
            entropy: false,
            memory_guard: false,
            shellcode: false,
            emulate_winapi: false,
            emulate_winapi_once: false,
            // On Linux, emulate the real libc/ld.so by default (executing the
            // genuine interpreter + libc, intercepting only at `syscall`).
            // Falls back to the in-Rust libc hooks when the host interpreter
            // can't be located.
            linux_real_libc: true,
            max_alloc_size: 0xffffff, // 16MB default
            module_name: constants::MODULE_NAME.to_string(),
            exe_name: constants::EXE_NAME.to_string(),
            user_name: constants::USER_NAME.to_string(),
            host_name: constants::HOST_NAME.to_string(),
            temp_path: constants::TEMP_PATH.to_string(),
            cwd_path: constants::CWD_PATH.to_string(),
            windows_directory: constants::WINDOWS_DIRECTORY.to_string(),
            system_directory: constants::SYSTEM_DIRECTORY.to_string(),
            max_instructions: None,
            timeout_secs: None,
            max_faults: None,
            seed: None,
            heap_alloc_min_size: 0,
            short_circuit_sleep: false,
            allow_empty_code_blocks: false,
            ssdt_use_ldr_initialize_thunk: false,
            soft_exception_recovery: false,
        }
    }

    /// Returns true when the configured architecture is x86-64.
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        self.arch.is_x64()
    }

    /// Returns true when the configured architecture is AArch64.
    #[inline(always)]
    pub fn is_aarch64(&self) -> bool {
        self.arch.is_aarch64()
    }

    /// Returns true when the configured architecture is 32-bit x86.
    #[inline(always)]
    pub fn is_x86(&self) -> bool {
        self.arch.is_x86()
    }

    /// Build an absolute path inside the configured maps folder.
    pub fn get_maps_folder(&self, filename: &str) -> String {
        let mut path = self.maps_folder.clone();
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(filename);
        path
    }
}

// --- Builder-style setters ---
//
// Every setter returns `&mut Self` so calls can be chained:
//
// ```rust
// let cfg = Config::new();
// cfg.set_filename("sample.exe")
//    .set_arch(Arch::X86_64)
//    .set_verbose(2);
// ```
impl Config {
    // -- Binary & architecture --

    /// Set the binary filename (full path).
    pub fn set_filename(&mut self, f: impl Into<String>) -> &mut Self {
        self.filename = f.into();
        self
    }

    /// Set the CPU architecture.
    pub fn set_arch(&mut self, a: Arch) -> &mut Self {
        self.arch = a;
        self
    }

    /// Enable or disable shellcode mode.
    pub fn set_shellcode(&mut self, enabled: bool) -> &mut Self {
        self.shellcode = enabled;
        self
    }

    /// Set command-line arguments passed to the emulated binary.
    pub fn set_arguments(&mut self, args: impl Into<String>) -> &mut Self {
        self.arguments = args.into();
        self
    }

    // -- Memory layout --

    /// Set the base address where code is loaded.
    pub fn set_code_base_addr(&mut self, addr: u64) -> &mut Self {
        self.code_base_addr = addr;
        self
    }

    /// Set the stack base address.
    pub fn set_stack_addr(&mut self, addr: u64) -> &mut Self {
        self.stack_addr = addr;
        self
    }

    /// Set the entry point address.
    pub fn set_entry_point(&mut self, addr: u64) -> &mut Self {
        self.entry_point = addr;
        self
    }

    /// Set the folder containing memory-mapped files (DLL maps, etc.).
    pub fn set_maps_folder(&mut self, folder: impl Into<String>) -> &mut Self {
        self.maps_folder = folder.into();
        self
    }

    /// Set the Windows version string for DLL resolution.
    pub fn set_winver(&mut self, ver: Option<String>) -> &mut Self {
        self.winver = ver;
        self
    }

    // -- Heap behavior --

    /// Set the maximum allocation size (larger requests are truncated).
    pub fn set_max_alloc_size(&mut self, size: u64) -> &mut Self {
        self.max_alloc_size = size;
        self
    }

    /// Set the minimum padding for all heap allocations.
    pub fn set_heap_alloc_min_size(&mut self, size: u64) -> &mut Self {
        self.heap_alloc_min_size = size;
        self
    }

    // -- Tracing --

    /// Enable or disable memory operation tracing.
    pub fn set_trace_mem(&mut self, enabled: bool) -> &mut Self {
        self.trace_mem = enabled;
        self
    }

    /// Enable or disable call tracing.
    pub fn set_trace_calls(&mut self, enabled: bool) -> &mut Self {
        self.trace_calls = enabled;
        self
    }

    /// Enable or disable full register dump on every step.
    pub fn set_trace_regs(&mut self, enabled: bool) -> &mut Self {
        self.trace_regs = enabled;
        self
    }

    /// Enable tracing of specific registers by name.
    pub fn set_trace_reg(&mut self, enabled: bool, names: Vec<String>) -> &mut Self {
        self.trace_reg = enabled;
        self.reg_names = names;
        self
    }

    /// Trace a string at the given address (0 disables).
    pub fn set_trace_string(&mut self, addr: u64) -> &mut Self {
        self.trace_string = addr != 0;
        self.string_addr = addr;
        self
    }

    /// Enable or disable flags tracing.
    pub fn set_trace_flags(&mut self, enabled: bool) -> &mut Self {
        self.trace_flags = enabled;
        self
    }

    /// Set the trace output filename (None disables file tracing).
    pub fn set_trace_filename(&mut self, filename: Option<String>) -> &mut Self {
        self.trace_filename = filename;
        self
    }

    /// Set the instruction position at which tracing begins.
    pub fn set_trace_start(&mut self, pos: u64) -> &mut Self {
        self.trace_start = pos;
        self
    }

    /// Enable or disable stack operation tracing.
    pub fn set_stack_trace(&mut self, enabled: bool) -> &mut Self {
        self.stack_trace = enabled;
        self
    }

    // -- Verbosity & output --

    /// Set the verbosity level (0..3).
    pub fn set_verbose(&mut self, level: u32) -> &mut Self {
        self.verbose = level;
        self
    }

    /// Set a position at which verbosity activates.
    pub fn set_verbose_at(&mut self, pos: Option<u64>) -> &mut Self {
        self.verbose_at = pos;
        self
    }

    /// Set a position range for verbose output.
    pub fn set_verbose_range(&mut self, start: u64, end: u64) -> &mut Self {
        self.verbose_start = start;
        self.verbose_end = end;
        self
    }

    /// Enable or disable colored output.
    pub fn set_nocolors(&mut self, enabled: bool) -> &mut Self {
        self.nocolors = enabled;
        self
    }

    // -- Console / interactive debugger --

    /// Spawn the console at instruction count `num`.
    pub fn set_console_at_pos(&mut self, num: u64) -> &mut Self {
        self.console = true;
        self.console_enabled = true;
        self.console_num = num;
        self
    }

    /// Spawn the console when execution reaches `addr`.
    pub fn set_console_at_addr(&mut self, addr: u64) -> &mut Self {
        self.console2 = true;
        self.console_enabled = true;
        self.console_addr = addr;
        self
    }

    /// Enable or disable console operations globally.
    pub fn set_console_enabled(&mut self, enabled: bool) -> &mut Self {
        self.console_enabled = enabled;
        self
    }

    // -- Execution limits --

    /// Set the instruction position at which emulation stops.
    pub fn set_exit_position(&mut self, pos: u64) -> &mut Self {
        self.exit_position = pos;
        self
    }

    /// Set the maximum number of instructions to execute.
    pub fn set_max_instructions(&mut self, max: Option<u64>) -> &mut Self {
        self.max_instructions = max;
        self
    }

    /// Set a wall-clock timeout in seconds.
    pub fn set_timeout_secs(&mut self, secs: Option<f64>) -> &mut Self {
        self.timeout_secs = secs;
        self
    }

    /// Set the maximum number of faults before stopping.
    pub fn set_max_faults(&mut self, faults: Option<u32>) -> &mut Self {
        self.max_faults = faults;
        self
    }

    /// Seed the emulator's random source (`rdrand`/`rdseed`) so a run is
    /// reproducible. Applied when emulation starts; `None` seeds from the OS.
    pub fn set_seed(&mut self, seed: Option<u64>) -> &mut Self {
        self.seed = seed;
        self
    }

    /// Enable or disable loop iteration counting (slow).
    pub fn set_loops(&mut self, enabled: bool) -> &mut Self {
        self.loops = enabled;
        self
    }

    /// Allow or disallow empty code blocks.
    pub fn set_allow_empty_code_blocks(&mut self, enabled: bool) -> &mut Self {
        self.allow_empty_code_blocks = enabled;
        self
    }

    // -- API emulation behavior --

    /// Enable or disable Windows API emulation.
    pub fn set_emulate_winapi(&mut self, enabled: bool) -> &mut Self {
        self.emulate_winapi = enabled;
        self
    }

    /// Enable or disable real libc execution on Linux.
    pub fn set_linux_real_libc(&mut self, enabled: bool) -> &mut Self {
        self.linux_real_libc = enabled;
        self
    }

    /// Skip unimplemented API calls instead of stopping.
    pub fn set_skip_unimplemented(&mut self, enabled: bool) -> &mut Self {
        self.skip_unimplemented = enabled;
        self
    }

    /// Make Sleep/Wait calls return immediately.
    pub fn set_short_circuit_sleep(&mut self, enabled: bool) -> &mut Self {
        self.short_circuit_sleep = enabled;
        self
    }

    // -- Threading --

    /// Enable or disable thread scheduling.
    pub fn set_enable_threading(&mut self, enabled: bool) -> &mut Self {
        self.enable_threading = enabled;
        self
    }

    // -- Memory inspection --

    /// Set a memory operand to watch during execution (empty string disables).
    pub fn set_inspect(&mut self, seq: impl Into<String>) -> &mut Self {
        let s = seq.into();
        self.inspect = !s.is_empty();
        self.inspect_seq = s;
        self
    }

    // -- Network emulation --

    /// Enable or disable network endpoint emulation.
    pub fn set_endpoint(&mut self, enabled: bool) -> &mut Self {
        self.endpoint = enabled;
        self
    }

    // -- Analysis --

    /// Enable or disable entropy measurement.
    pub fn set_entropy(&mut self, enabled: bool) -> &mut Self {
        self.entropy = enabled;
        self
    }

    /// Enable or disable heap memory-safety analysis.
    pub fn set_memory_guard(&mut self, enabled: bool) -> &mut Self {
        self.memory_guard = enabled;
        self
    }

    // -- Dump on exit --

    /// Enable or disable memory dump on exit.
    pub fn set_dump_on_exit(&mut self, enabled: bool) -> &mut Self {
        self.dump_on_exit = enabled;
        self
    }

    /// Set the dump output filename.
    pub fn set_dump_filename(&mut self, filename: Option<String>) -> &mut Self {
        self.dump_filename = filename;
        self
    }

    // -- Simulated environment constants --

    /// Set the emulated module name.
    pub fn set_module_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.module_name = name.into();
        self
    }

    /// Set the emulated executable name.
    pub fn set_exe_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.exe_name = name.into();
        self
    }

    /// Set the emulated user name.
    pub fn set_user_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.user_name = name.into();
        self
    }

    /// Set the emulated host name.
    pub fn set_host_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.host_name = name.into();
        self
    }

    /// Set the emulated temp directory path.
    pub fn set_temp_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.temp_path = path.into();
        self
    }

    /// Set the emulated current working directory.
    pub fn set_cwd_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.cwd_path = path.into();
        self
    }

    /// Set the emulated Windows directory path.
    pub fn set_windows_directory(&mut self, path: impl Into<String>) -> &mut Self {
        self.windows_directory = path.into();
        self
    }

    /// Set the emulated System32 directory path.
    pub fn set_system_directory(&mut self, path: impl Into<String>) -> &mut Self {
        self.system_directory = path.into();
        self
    }

    // -- Other --

    /// Use ntdll LdrInitializeThunk for PE64 loader init.
    pub fn set_ssdt_use_ldr_initialize_thunk(&mut self, enabled: bool) -> &mut Self {
        self.ssdt_use_ldr_initialize_thunk = enabled;
        self
    }

    /// Continue past unhandled memory faults instead of terminating.
    pub fn set_soft_exception_recovery(&mut self, enabled: bool) -> &mut Self {
        self.soft_exception_recovery = enabled;
        self
    }

    /// Enable or disable test mode.
    pub fn set_test_mode(&mut self, enabled: bool) -> &mut Self {
        self.test_mode = enabled;
        self
    }
}
