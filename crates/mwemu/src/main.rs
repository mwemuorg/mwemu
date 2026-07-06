#![allow(unexpected_cfgs)]

extern crate clap;

use clap::{App, Arg};
use libmwemu::emu32;
use libmwemu::emu64;
use libmwemu::emu_aarch64;
use libmwemu::serialization;
use std::{panic, process};
use std::path::PathBuf;
//use libmwemu::definitions;
use fast_log::appender::{Command, FastLogRecord, RecordFormat};
use fast_log::filter::Filter;
use fast_log::Config;
use libmwemu::disable_color;
use libmwemu::emu::object_handle::file_handle::init_file_system;

/// Drop log records emitted by `goblin::*` (Mach-O / ELF / PE parser). It uses
/// `debug!()` to dump every load command, header, and Ctx — under `-vv` (which
/// raises our level to TRACE) those drown out our own emulator trace lines.
struct DropNoisyDepsFilter;

impl Filter for DropNoisyDepsFilter {
    fn do_log(&self, record: &log::Record) -> bool {
        let m = record.module_path().unwrap_or("");
        // Drop chatty third-party crates so they don't bury mwemu's own output:
        // goblin (PE parsing) and the HTTPS/TLS stack behind `--winver` fetches
        // (ureq/rustls/ring/webpki spam OCSP responses and HTTP headers at debug).
        const NOISY: &[&str] = &[
            "goblin", "ureq", "rustls", "webpki", "ring", "native_tls", "openssl",
            "hyper", "h2", "mio", "want", "tungstenite", "sct", "http", "httparse",
            "tokio",
        ];
        !NOISY.iter().any(|p| m.starts_with(p))
    }
}

macro_rules! match_register_arg {
    ($matches:expr, $emu:expr, $reg:expr) => {
        if $matches.is_present($reg) {
            let value = u64::from_str_radix(
                $matches
                    .value_of($reg)
                    .expect(concat!("select the ", $reg, " register"))
                    .trim_start_matches("0x"),
                16,
            )
            .expect("invalid register value");
            $emu.regs_mut().set_reg_by_name($reg, value);
        }
    };
}

macro_rules! clap_arg {
    // Basic argument with just name, short, long, and help
    ($name:expr, $short:expr, $long:expr, $help:expr) => {
        Arg::with_name($name)
            .short($short)
            .long($long)
            .help($help)
            .takes_value(false)
    };

    // Argument that takes a value
    ($name:expr, $short:expr, $long:expr, $help:expr, $value_name:expr) => {
        Arg::with_name($name)
            .short($short)
            .long($long)
            .help($help)
            .takes_value(true)
            .value_name($value_name)
    };

    // Multiple flag variant (using true/false as explicit boolean)
    ($name:expr, $short:expr, $long:expr, $help:expr, multiple: $multiple:expr) => {
        Arg::with_name($name)
            .short($short)
            .long($long)
            .help($help)
            .multiple($multiple)
            .takes_value(false)
    };
}

pub struct CustomLogFormat {}

impl RecordFormat for CustomLogFormat {
    fn do_format(&self, arg: &mut FastLogRecord) {
        match &arg.command {
            Command::CommandRecord => {
                arg.formated = format!("{}\n", arg.args);
            }
            Command::CommandExit => {}
            Command::CommandFlush(_) => {}
        }
    }
}

impl CustomLogFormat {
    pub fn new() -> CustomLogFormat {
        Self {}
    }
}

fn main() {
    let matches = App::new("MWEMU emulator for malware")
        .version(env!("CARGO_PKG_VERSION"))
        .author("@sha0coder")
        .arg(clap_arg!("filename", "f", "filename", "set the shellcode binary file.", "FILE"))
        .arg(clap_arg!("dump", "d", "dump", "load from dump.", "FILE"))
        .arg(clap_arg!("verbose", "v", "verbose", "-v syscalls+api calls, -vv assembly, -vvv reps; default shows only syscalls", multiple: true))
        .arg(clap_arg!("verbose_at", "V", "verbose_at", "enable assembly display from position N (-V N) or between positions (-V N-M)", "RANGE"))
        .arg(clap_arg!("64bits", "6", "64bits", "enable 64bits architecture emulation"))
        .arg(clap_arg!("aarch64", "", "aarch64", "enable AArch64/ARM64 architecture emulation"))
        .arg(clap_arg!("trace_memory", "m", "trace_memory", "trace all the memory accesses read and write."))
        .arg(clap_arg!("flags", "", "flags", "trace the flags hex value in every instruction."))
        .arg(clap_arg!("maps", "M", "maps", "select the memory maps folder", "PATH"))
        .arg(clap_arg!("iso", "", "iso", "extract the genuine system32 DLLs from a Windows installation ISO (download it yourself) and use them as the maps folder, e.g. --iso ~/Downloads/win11.iso", "ISO"))
        .arg(clap_arg!("winver", "", "winver", "fetch genuine system DLLs from Microsoft's symbol server on demand instead of an ISO; accepts a friendly name (win11, win10, win2022) or an exact build (26100.7920). e.g. --winver win11", "VERSION"))
        .arg(clap_arg!("trace_registers", "r", "trace_registers", "print the register values in every step."))
        .arg(clap_arg!("register", "R", "trace_register", "trace a specific register in every step, value and content", "REGISTER1,REGISTER2"))
        .arg(clap_arg!("console", "c", "console", "select in which moment will spawn the console to inspect.", "NUMBER"))
        .arg(clap_arg!("loops", "l", "loops", "show loop iterations, it is slow."))
        .arg(clap_arg!("nocolors", "n", "nocolors", "print without colors for redirecting to a file >out"))
        .arg(clap_arg!("string", "s", "string", "monitor string on a specific address", "ADDRESS"))
        .arg(clap_arg!("inspect", "i", "inspect", "monitor memory like: -i 'dword ptr [ebp + 0x24]", "DIRECTION"))
        //.arg(clap_arg!("endpoint", "e", "endpoint", "perform communications with the endpoint, use tor or vpn!"))
        .arg(clap_arg!("console_addr", "C", "console_addr", "spawn console on first eip = address", "ADDRESS"))
        .arg(clap_arg!("entry_point", "a", "entry", "entry point of the shellcode, by default starts from the beginning.", "ADDRESS"))
        .arg(clap_arg!("exit_position", "e", "exit", "exit position of the shellcode", "POSITION"))
        .arg(clap_arg!("code_base_address", "b", "base", "set base address for code", "ADDRESS"))
        .arg(clap_arg!("stack_address", "", "stack_address", "set stack address", "ADDRESS"))
        .arg(clap_arg!("handle", "h", "handle", "handle Ctrl+C to spawn console"))
        .arg(clap_arg!("stack_trace", "p", "stack_trace", "trace stack on push/pop"))
        .arg(clap_arg!("test_mode", "t", "test", "test mode"))
        .arg(clap_arg!("banzai", "", "banzai", "skip unimplemented instructions, and keep up emulating what can be emulated"))
        .arg(clap_arg!("script", "x", "script", "launch an emulation script, see scripts_examples folder", "SCRIPT"))
        .arg(clap_arg!("args", "A", "args", "provide arguments to the EXE like: --args '\"aa\" \"bb\"'", "ARGS"))
        .arg(clap_arg!("trace_filename", "T", "trace_filename", "output trace to specified file", "TRACE_FILENAME"))
        .arg(clap_arg!("trace_start", "S", "trace_start", "start trace at specified position", "TRACE_START"))
        .arg(clap_arg!("log","L", "log", "log output to file", "LOG_FILENAME")) 
        .arg(clap_arg!("fpu","F", "fpu", "trace the fpu states."))
        .arg(clap_arg!("rax", "", "rax", "set rax register", "RAX"))
        .arg(clap_arg!("rbx", "", "rbx", "set rbx register", "RBX"))
        .arg(clap_arg!("rcx", "", "rcx", "set rcx register", "RCX"))
        .arg(clap_arg!("rdx", "", "rdx", "set rdx register", "RDX"))
        .arg(clap_arg!("rsp", "", "rsp", "set rsp register", "RSP"))
        .arg(clap_arg!("rbp", "", "rbp", "set rbp register", "RBP"))
        .arg(clap_arg!("rip", "", "rip", "set rip register", "RIP"))
        .arg(clap_arg!("rsi", "", "rsi", "set rsi register", "RSI"))
        .arg(clap_arg!("rdi", "", "rdi", "set rdi register", "RDI"))
        .arg(clap_arg!("r8", "", "r8", "set r8 register", "R8"))
        .arg(clap_arg!("r9", "", "r9", "set r9 register", "R9"))
        .arg(clap_arg!("r10", "", "r10", "set r10 register", "R10"))
        .arg(clap_arg!("r11", "", "r11", "set r11 register", "R11"))
        .arg(clap_arg!("r12", "", "r12", "set r12 register", "R12"))
        .arg(clap_arg!("r13", "", "r13", "set r13 register", "R13"))
        .arg(clap_arg!("r14", "", "r14", "set r14 register", "R14"))
        .arg(clap_arg!("r15", "", "r15", "set r15 register", "R15"))
        .arg(clap_arg!("rflags", "", "rflags", "set rflags register", "RFLAGS"))
        .arg(clap_arg!("mxcsr", "", "mxcsr", "set mxcsr register", "MXCSR"))
        .arg(clap_arg!("call", "", "call", "enable call tracer"))
        .arg(clap_arg!("cmd", "", "cmd", "launch a console command", "COMMAND"))
        .arg(clap_arg!("entropy", "", "entropy", "display changes in the entropy"))
        .arg(clap_arg!("multithread", "", "multithread", "enable multithread emulation"))
        .arg(clap_arg!("is_shellcode", "", "is_shellcode", "Force the binary to be shellcode"))
        // `--ssdt` runs real PE bytes (ntdll, kernel32, kernelbase, …) and
        // intercepts only at the `syscall` instruction — the opposite of the
        // default API-stub path. `--syscall-mode` is a more descriptive alias.
        .arg(
            Arg::with_name("ssdt")
                .long("ssdt")
                .alias("syscall-mode")
                .help(
                    "run real DLL PE bytes and intercept only at syscalls \
                     (alias: --syscall-mode). Slower but more accurate against \
                     anti-emulation checks.",
                )
                .takes_value(false),
        )
        // `--libc` is the Linux counterpart of `--ssdt`: it maps the real
        // libc/ld code and executes it, intercepting only at the `syscall`
        // instruction instead of hooking each libc function in Rust.
        .arg(
            Arg::with_name("libc")
                .long("libc")
                .alias("linux-syscall-mode")
                .help(
                    "run the real mapped libc/ld code and intercept only at \
                     syscalls (alias: --linux-syscall-mode). Linux counterpart \
                     of --ssdt.",
                )
                .takes_value(false),
        )
        .arg(clap_arg!("gdb", "g", "gdb", "enable GDB remote debugging server"))
        .arg(clap_arg!("gdb_port", "P", "gdb-port", "set GDB server port (default: 9001)", "PORT"))
        .arg(clap_arg!("gdb_wait", "W", "gdb-wait", "wait for GDB connection before starting"))
        .get_matches();

    if !matches.is_present("filename") {
        log::error!("the filename is mandatory, try -f <FILENAME> or --help");
        return;
    }

    let mut emu: libmwemu::emu::Emu;

    // Architecture selection
    if matches.is_present("aarch64") {
        emu = emu_aarch64();
    } else if matches.is_present("64bits") {
        emu = emu64();
    } else {
        emu = emu32();
    }

    emu.running_script = false;

    // `--handle`: Ctrl-C → spawn console. Set this BEFORE any emulation runs
    // (in SSDT mode `load_code` already runs ntdll!LdrInitializeThunk, and the
    // global current-emu snapshot is taken before load), so the handler covers
    // the whole run including init.
    if !matches.is_present("script") && !matches.is_present("gdb") && matches.is_present("handle") {
        emu.cfg.console_enabled = true;
        emu.enable_ctrlc();
    }

    // filename
    let filename = matches
        .value_of("filename")
        .expect("the filename is mandatory, try -f <FILENAME> or --help.")
        .to_string();
    emu.cfg.filename = filename.clone();

    // verbose
    emu.cfg.verbose = matches.occurrences_of("verbose") as u32;
    emu.set_verbose(emu.cfg.verbose);
    if emu.cfg.verbose == 0 {
        log::info!("use -vv to see the assembly code emulated, and -v to see the messages");
    }

    // tracing
    emu.cfg.trace_mem = matches.is_present("trace_memory");
    emu.cfg.trace_regs = matches.is_present("trace_registers");
    emu.cfg.trace_calls = matches.is_present("call");

    if matches.is_present("entropy") {
        emu.cfg.entropy = true;
    }

    if matches.is_present("register") {
        emu.cfg.trace_reg = true;
        let regs: String = matches
            .value_of("register")
            .expect("select the register example: eax,ebx")
            .to_string();
        emu.cfg.reg_names = regs.split(',').map(|x| x.to_string()).collect();
    }

    if matches.is_present("string") {
        emu.cfg.trace_string = true;
        emu.cfg.string_addr = u64::from_str_radix(
            matches
                .value_of("string")
                .expect("select the address of the string")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
    }
    if matches.is_present("trace_filename") {
        let trace_filename = matches
            .value_of("trace_filename")
            .expect("specify the trace output file")
            .to_string();
        emu.cfg.trace_filename = Some(trace_filename);
        emu.open_trace_file();
    }
    if matches.is_present("trace_start") {
        emu.cfg.trace_start = u64::from_str_radix(
            matches
                .value_of("trace_start")
                .expect("select the trace start address")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
    }

    // emulate winapi via syscall dispatcher (currently x64 only)
    if matches.is_present("ssdt") {
        if !emu.cfg.is_x64() {
            panic!("SSDT mode is not supported in 32-bit mode yet.");
        }
        emu.cfg.emulate_winapi = true;
    }

    // Linux real-libc execution mode (counterpart of --ssdt).
    if matches.is_present("libc") {
        if !emu.cfg.is_x64() {
            panic!("--libc real-libc mode is only supported in 64-bit mode yet.");
        }
        emu.cfg.linux_real_libc = true;
    }
    // -V N        → enable verbose from step N onwards (permanent)
    // -V N-M      → enable verbose only between steps N and M (range, restores afterwards)
    if matches.is_present("verbose_at") {
        let range_str = matches
            .value_of("verbose_at")
            .expect("select a position like 123 or a range like 123-456");
        let parts: Vec<&str> = range_str.splitn(2, '-').collect();
        let start = parts[0]
            .trim()
            .parse::<u64>()
            .expect("-V: invalid start position");
        if parts.len() == 2 && !parts[1].trim().is_empty() {
            // Range mode: verbose only between start and end.
            let end = parts[1]
                .trim()
                .parse::<u64>()
                .expect("-V: invalid end position");
            emu.cfg.verbose_start = start;
            emu.cfg.verbose_end = end;
        } else {
            // Single position: enable verbose permanently from start.
            emu.cfg.verbose_at = Some(start);
        }
    }

    // console
    if matches.is_present("console") {
        emu.cfg.console = true;
        emu.cfg.console_enabled = true;
        emu.cfg.console_num = matches
            .value_of("console")
            .expect("select the number of moment to inspect")
            .parse::<u64>()
            .expect("select a valid number to spawn console");
        emu.spawn_console_at(emu.cfg.console_num);
        if emu.cfg.console_num > 25 {
            emu.cfg.verbose_at = Some(emu.cfg.console_num - 25);
        }
    }
    emu.cfg.loops = matches.is_present("loops");
    emu.cfg.nocolors = matches.is_present("nocolors");

    // inspect
    if matches.is_present("inspect") {
        emu.cfg.inspect = true;
        emu.cfg.inspect_seq = matches
            .value_of("inspect")
            .expect("select the address in the way 'dword ptr [eax + 0xa]'")
            .to_string();
    }

    // banzai
    if matches.is_present("banzai") {
        emu.cfg.skip_unimplemented = true;
        emu.maps.set_banzai(true);
    }

    // A non-Windows guest (ELF / Mach-O) never touches the Windows system DLLs,
    // so skip the winver fetch (including the default win11 one) entirely. We ask
    // libmwemu to classify with the very same detectors `load_code` uses, so a
    // shellcode whose first bytes happen to look like an ELF isn't misjudged, and
    // --is_shellcode still forces the Windows shellcode path.
    let is_non_windows_target =
        libmwemu::emu::Emu::is_non_windows_file(&filename, matches.is_present("is_shellcode"));

    // maps
    if matches.is_present("iso") {
        let iso = matches.value_of("iso").expect("specify the ISO path");
        // The logger isn't initialized yet here and extracting from a multi-GB
        // ISO is slow, so talk to the user directly on stderr.
        eprintln!("[mwemu] extracting genuine system32 from {} ...", iso);
        match emu.set_maps_from_iso(iso) {
            Ok(()) => eprintln!("[mwemu] system32 ready: {}", emu.cfg.maps_folder),
            Err(e) => {
                eprintln!("[mwemu] --iso failed: {}", e);
                std::process::exit(1);
            }
        }
    } else if is_non_windows_target {
        // Linux/macOS guest: no Windows system DLLs needed. Honor an explicit
        // --maps if the user gave one, otherwise leave maps untouched — the ELF/
        // Mach-O loader maps the binary's own segments.
        if matches.is_present("maps") {
            emu.set_maps_folder(matches.value_of("maps").expect("specify the maps folder"));
        }
    } else if matches.is_present("winver") {
        let winver = matches.value_of("winver").expect("specify the Windows version");
        match emu.set_maps_from_winver(winver) {
            Ok(()) => eprintln!("[mwemu] system32 ready: {}", emu.cfg.maps_folder),
            Err(e) => {
                eprintln!("[mwemu] --winver failed: {}", e);
                std::process::exit(1);
            }
        }
    } else if matches.is_present("maps") {
        emu.set_maps_folder(matches.value_of("maps").expect("specify the maps folder"));
    } else {
        // No --iso/--winver/--maps: default to genuine win11 DLLs from the
        // symbol server (same as `--winver win11`), for both 64- and 32-bit
        // guests, so the emulator runs against a real, current ntdll/kernelbase
        // instead of the stale bundled maps. set_maps_from_winver picks the
        // architecture from cfg.arch (already set above). If the fetch fails
        // (e.g. offline), fall back to the legacy bundled maps so it still runs.
        let legacy = if emu.cfg.is_x64() {
            "maps/windows/x86_64/"
        } else {
            "maps/windows/x86/"
        };
        match emu.set_maps_from_winver("win11") {
            Ok(()) => eprintln!("[mwemu] system32 ready: {}", emu.cfg.maps_folder),
            Err(e) => {
                eprintln!(
                    "[mwemu] default --winver win11 failed ({}); falling back to bundled maps",
                    e
                );
                emu.set_maps_folder(legacy);
            }
        }
    }

    // code base address
    if matches.is_present("code_base_address") {
        emu.cfg.code_base_addr = u64::from_str_radix(
            matches
                .value_of("code_base_address")
                .expect("select the code base address -b")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
        if !matches.is_present("entry_point") {
            log::error!("if the code base is selected, you have to select the entry point ie -b 0x600000 -a 0x600000");
            std::process::exit(1);
        }
    }

    // stack address
    if matches.is_present("stack_address") {
        emu.cfg.stack_addr = u64::from_str_radix(
            matches
                .value_of("stack_address")
                .expect("select the stack address")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
    }

    // register values
    match_register_arg!(matches, emu, "rax");
    match_register_arg!(matches, emu, "rbx");
    match_register_arg!(matches, emu, "rcx");
    match_register_arg!(matches, emu, "rdx");
    match_register_arg!(matches, emu, "rsp");
    match_register_arg!(matches, emu, "rip");
    match_register_arg!(matches, emu, "rbp");
    match_register_arg!(matches, emu, "rsi");
    match_register_arg!(matches, emu, "rdi");
    match_register_arg!(matches, emu, "r8");
    match_register_arg!(matches, emu, "r9");
    match_register_arg!(matches, emu, "r10");
    match_register_arg!(matches, emu, "r11");
    match_register_arg!(matches, emu, "r12");
    match_register_arg!(matches, emu, "r13");
    match_register_arg!(matches, emu, "r14");
    match_register_arg!(matches, emu, "r15");
    if matches.is_present("rflags") {
        let value = u64::from_str_radix(
            matches
                .value_of("rflags")
                .expect("select the rflags register")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
        emu.flags_mut().load(value as u32);
    }
    if matches.is_present("mxcsr") {
        let value = u64::from_str_radix(
            matches
                .value_of("mxcsr")
                .expect("select the mxcsr register")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
        emu.fpu_mut().mxcsr = value as u32;
    }

    if matches.is_present("multithread") {
        emu.cfg.enable_threading = true;
    }

    // endpoint
    if matches.is_present("endpoint") {
        //TODO: emu::endpoint::warning();
        emu.cfg.endpoint = true;
    }

    // console
    if matches.is_present("console_addr") {
        emu.cfg.console2 = true;
        emu.cfg.console_enabled = true;
        emu.cfg.console_addr = u64::from_str_radix(
            matches
                .value_of("console_addr")
                .expect("select the address to spawn console with -C")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
        emu.spawn_console_at_addr(emu.cfg.console_addr);
    }

    // entry point
    if matches.is_present("entry_point") {
        emu.cfg.entry_point = u64::from_str_radix(
            matches
                .value_of("entry_point")
                .expect("select the entry point address -a")
                .trim_start_matches("0x"),
            16,
        )
        .expect("invalid address");
    }

    // exit position
    if matches.is_present("exit_position") {
        let exit_pos_str = matches
            .value_of("exit_position")
            .expect("select the exit position address -e");

        emu.cfg.exit_position = if exit_pos_str.starts_with("0x") {
            // Handle hexadecimal format
            u64::from_str_radix(exit_pos_str.trim_start_matches("0x"), 16)
        } else {
            // Handle decimal format
            exit_pos_str.parse::<u64>()
        }
        .expect("invalid position");
    }

    // stack trace (push/pop logging in stack.rs)
    if matches.is_present("stack_trace") {
        emu.cfg.stack_trace = true;
    }

    // test mode
    if matches.is_present("test_mode") {
        emu.cfg.test_mode = true;
    }

    // trace fpu
    if matches.is_present("fpu") {
        emu.fpu_mut().trace = true;
    }

    // trace flags
    if matches.is_present("flags") {
        emu.cfg.trace_flags = true;
    }

    // cmd
    if matches.is_present("cmd") {
        emu.cfg.command = Some(
            matches
                .value_of("cmd")
                .expect("specify the console command")
                .to_string(),
        );
    }

    if matches.is_present("is_shellcode") {
        emu.cfg.shellcode = true;
    }

    // args
    if matches.is_present("args") {
        log::info!(
            "espeicificando argumentos: {}",
            matches
                .value_of("args")
                .expect("specify the argument string")
                .to_string()
        );
        emu.cfg.arguments = matches
            .value_of("args")
            .expect("specify the argument string")
            .to_string();
    }

    // log to file
    if matches.is_present("log") {
        let filename = matches.value_of("log").expect("log filename is missing");
        // `colors.disable()` alone is not enough: `show_instruction` / macros use `cfg.nocolors`,
        // and `color!()` uses the global `color_enabled()` flag — both must be off for plain text.
        emu.cfg.nocolors = true;
        disable_color();
        emu.colors.disable();
        fast_log::init(
            Config::new()
                .format(CustomLogFormat::new())
                .file(filename)
                .chan_len(Some(100000))
                .add_filter(DropNoisyDepsFilter),
        )
        .unwrap();
    } else {
        fast_log::init(
            Config::new()
                .format(CustomLogFormat::new())
                .console()
                .chan_len(Some(100000))
                .add_filter(DropNoisyDepsFilter),
        )
        .unwrap();
    }

    // definitions
    //emu.cfg.definitions = definitions::load_definitions("definitions/test.json");

    // setup hook to flush the log when end the program
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Try to log emulator state if available
        libmwemu::emu_context::with_current_emu(|emu| {
            // log state
            libmwemu::emu_context::log_emu_state(emu);

            // dump on exit
            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_ref().unwrap(),
                );
            }
        });

        // flush all the log
        log::logger().flush();
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    // set current
    libmwemu::emu_context::set_current_emu(&emu);

    // load code
    emu.load_code(&filename);

    // Initialize file system in order to call the create file stuff
    let result_ok = init_file_system(None as Option<PathBuf>);
    if result_ok.is_err() {
        log::error!("Cannot initialize file system (see error above).");
        return;
    }

    // override all from dump?
    if matches.is_present("dump") {
        let dump_filename = matches.value_of("dump").expect("specify the dump filename");
        log::info!("loading dump from {}", dump_filename);
        let old_config = emu.cfg;
        emu = serialization::Serialization::load(dump_filename);
        emu.cfg = old_config;
        emu.maps.set_banzai(emu.cfg.skip_unimplemented);
    }

    // script
    if matches.is_present("script") {
        emu.disable_ctrlc();
        let mut script = libmwemu::script::Script::new();
        script.load(
            matches
                .value_of("script")
                .expect("select a script filename"),
        );

        // Run script
        script.run(&mut emu);

        // Clear the current emu
        libmwemu::emu_context::clear_current_emu();
    } else if matches.is_present("gdb") {
        // GDB server mode
        let port: u16 = matches
            .value_of("gdb_port")
            .map(|p| p.parse().expect("invalid port number"))
            .unwrap_or(9001);

        log::info!("Starting GDB remote debugging server...");

        let mut server = libmwemu::gdb::GdbServer::new(port, emu.cfg.arch);
        match server.run(&mut emu) {
            Ok(()) => {
                log::info!("GDB session ended normally");
            }
            Err(e) => {
                log::error!("GDB server error: {}", e);
            }
        }

        // Clear the current emu
        libmwemu::emu_context::clear_current_emu();
        log::logger().flush();
    } else {
        // `--handle` was already applied before load_code (Ctrl-C → console must
        // cover the SSDT LdrInitializeThunk init that runs during load).
        let result = emu.run(None);

        // Dump registers/stack like the panic hook — run() returns Err without panicking.
        if let Err(ref e) = result {
            if e.message != "empty code block" {
                libmwemu::emu_context::log_emu_state(&mut emu);
            }
        }

        // Clear the current emu
        libmwemu::emu_context::clear_current_emu();

        log::logger().flush();
        if let Err(e) = result {
            let msg = e.to_string();
            if e.message != "empty code block" {
                log::error!("{}", msg);
            }
            process::exit(1);
        }
    }
}
