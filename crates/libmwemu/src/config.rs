use std::collections::HashMap;

use crate::arch::Arch;
use crate::{debug::definitions::Definition, windows::constants};
use serde::{Deserialize, Serialize};

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
    pub heap_free_soft: bool, // mark memory as freed but don't deallocate

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
    pub loops: bool,                   // count iterations per instruction (slow)
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

    // --- Testing ---
    pub test_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
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
            shellcode: false,
            emulate_winapi: false,
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
            short_circuit_sleep: false,
            heap_alloc_min_size: 0,
            heap_free_soft: false,
            allow_empty_code_blocks: false,
            ssdt_use_ldr_initialize_thunk: false,
        }
    }

    pub fn is_x64(&self) -> bool {
        self.arch.is_x64()
    }

    pub fn is_aarch64(&self) -> bool {
        self.arch.is_aarch64()
    }

    pub fn is_x86(&self) -> bool {
        self.arch.is_x86()
    }

    pub fn get_maps_folder(&self, filename: &str) -> String {
        let mut path = self.maps_folder.clone();
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(filename);
        path
    }
}
