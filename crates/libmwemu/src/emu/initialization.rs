use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::time::Instant;

use atty::Stream;
use csv::ReaderBuilder;
use iced_x86::{Formatter as _, IntelFormatter};
use std::collections::BTreeSet;

use crate::emu::disassemble::InstructionCache;
use crate::emu::{ArchState, Emu};
use crate::maps::mem64::Permission;
use rs_header::pe::pe64;
use crate::windows::peb::{peb32, peb64};
use crate::{
    api::banzai::Banzai, config::Config, debug::breakpoint::Breakpoints, hooks::Hooks, maps::Maps,
    threading::context::ThreadContext, threading::global_locks::GlobalLocks, utils::colors::Colors,
};
use crate::{winapi::winapi32, winapi::winapi64, windows::kuser_shared, windows::structures};

use crate::emu::object_handle::HandleManagement;
use crate::maps::heap_allocation::O1Heap;
use fast_log::appender::{Command, FastLogRecord, RecordFormat};

pub struct CustomLogFormat;
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

impl Default for Emu {
    fn default() -> Self {
        Emu::new(crate::arch::Arch::X86)
    }
}

pub struct Lib {
    pe64: pe64::PE64,
    /// Raw image bytes, kept because rs-header's PE is borrow-based and the
    /// later IAT/delay-load binding stage needs them.
    raw: Vec<u8>,
    base: u64,
    name: String,
}

impl Emu {
    pub fn new(arch: crate::arch::Arch) -> Emu {
        let arch_state = if arch.is_aarch64() {
            ArchState::AArch64 {
                instruction: None,
                instruction_cache: InstructionCache::new(),
            }
        } else {
            let mut formatter = IntelFormatter::new();
            formatter.options_mut().set_digit_separator("");
            formatter.options_mut().set_first_operand_char_index(6);
            ArchState::X86 {
                instruction: None,
                formatter,
                instruction_cache: InstructionCache::new(),
                decoder_position: 0,
            }
        };

        let mut cfg = Config::new();
        cfg.arch = arch;

        Emu {
            arch_state,
            maps: {
                let mut maps = Maps::default();
                maps.is_64bits = arch.is_64bits();
                maps
            },
            hooks: Hooks::new(),
            exp: 0,
            break_on_alert: false,
            bp: Breakpoints::new(),
            cfg,
            colors: Colors::new(),
            pos: 0,
            max_pos: None,
            force_break: false,
            process_terminated: false,
            call_depth: 0,
            ldr_init_done: false,
            force_reload: false,
            tls_callbacks: Vec::new(),
            main_thread_cont: 0,
            gateway_return: 0,
            is_running: Arc::new(AtomicU32::new(0)),
            ctrlc_console: Arc::new(AtomicU32::new(0)),
            break_on_next_cmp: false,
            break_on_next_return: false,
            filename: String::new(),
            enabled_ctrlc: false,
            run_until_ret: false,
            running_script: true,
            banzai: Banzai::new(),
            os: crate::arch::OperatingSystem::Windows,
            now: Instant::now(),
            skip_apicall: false,
            its_apicall: None,
            last_decoded: None,
            last_decoded_addr: 0,
            last_instruction_size: 0,
            pe64: None,
            pe32: None,
            pe64_raw: None,
            pe32_raw: None,
            elf64: None,
            elf32: None,
            macho64: None,
            memory_operations: vec![],
            rep: None,
            tick: 0,
            trace_file: None,
            base: 0,
            heap_addr: 0,
            rng: RefCell::new(rand::rng()),
            threads: vec![ThreadContext::new(0x1000, arch)],
            current_thread_id: 0,
            global_locks: GlobalLocks::new(),
            definitions: HashMap::new(),
            stored_contexts: HashMap::new(),
            entropy: 0.0,
            heap_management: None,
            last_error: 0,
            is_api_run: false,
            ld_bootstrap: false,
            is_break_on_api: false,
            instruction_count: 0,
            fault_count: 0,
            handle_management: HandleManagement::new(),
            library_loaded: false,
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
        }
    }

    /// This inits the 32bits stack, it's called from init_cpu() and init()
    pub fn init_stack32(&mut self) {
        // default if not set via clap args
        if self.cfg.stack_addr == 0 {
            self.cfg.stack_addr = 0x212000;
            let esp = self.cfg.stack_addr + 0x1c000 + 4;
            let ebp = self.cfg.stack_addr + 0x1c000 + 4 + 0x1000;
            self.regs_mut().set_esp(esp);
            self.regs_mut().set_ebp(ebp);
        }

        // Store register values in local variables
        let esp = self.regs().get_esp();
        let ebp = self.regs().get_ebp();

        let (stack_base, stack_bottom) = if let Some(stack) = self.maps.get_map_by_name("stack") {
            (stack.get_base(), stack.get_bottom())
        } else {
            let stack = self
                .maps
                .create_map(
                    "stack",
                    self.cfg.stack_addr,
                    0x030000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create stack map");
            (stack.get_base(), stack.get_bottom())
        };

        // Now do all the assertions using the local variables
        assert!(esp < ebp);
        assert!(esp > stack_base);
        assert!(esp < stack_bottom);
        assert!(ebp > stack_base);
        assert!(ebp < stack_bottom);
        assert!(esp >= stack_base && esp < stack_bottom);
        assert!(ebp >= stack_base && ebp < stack_bottom);
    }

    /// This inits the 64bits stack, it's called from init_cpu() and init()
    pub fn init_stack64(&mut self) {
        // Large stack only when booting through `ntdll!LdrInitializeThunk`.
        // 4 MB is enough for the LdrInitializeThunk path itself; the api-set
        // fallback for unresolved dependencies runs each DllMain once and
        // unwinds normally, so additional headroom isn't needed in practice.
        let stack_size = if self.cfg.emulate_winapi {
            0x400000
        } else {
            0x100000
        };

        // default if not set via clap args
        if self.cfg.stack_addr == 0 {
            self.cfg.stack_addr = if self.cfg.emulate_winapi {
                0x10000
            } else {
                0x22a000
            };
            self.regs_mut().rsp = self.cfg.stack_addr + stack_size;
            self.regs_mut().rbp = self.cfg.stack_addr + stack_size + 0x1000;
        }

        // Store register values in local variables
        let rsp = self.regs().rsp;
        let rbp = self.regs().rbp;

        // Add extra buffer beyond rbp to ensure it's strictly less than bottom
        let (stack_base, stack_bottom) = if let Some(stack) = self.maps.get_map_by_name("stack") {
            (stack.get_base(), stack.get_bottom())
        } else {
            let stack = self
                .maps
                .create_map(
                    "stack",
                    self.cfg.stack_addr,
                    stack_size + 0x2000,
                    Permission::READ_WRITE,
                ) // Increased size
                .expect("cannot create stack map");
            (stack.get_base(), stack.get_bottom())
        };

        // Now do all the assertions using the local variables
        assert!(rsp < rbp);
        assert!(rsp > stack_base);
        assert!(rsp < stack_bottom);
        assert!(rbp > stack_base);
        assert!(rbp < stack_bottom);
        assert!(rsp >= stack_base && rsp < stack_bottom);
        assert!(rbp >= stack_base && rbp < stack_bottom);
    }

    //TODO: tests only in tests.rs
    pub fn init_stack_aarch64(&mut self) {
        let stack_size: u64 = 0x100000;

        if self.cfg.stack_addr == 0 {
            self.cfg.stack_addr = 0x22a000;
        }

        let sp = self.cfg.stack_addr + stack_size;

        if self.maps.get_map_by_name("stack").is_none() {
            self.maps
                .create_map(
                    "stack",
                    self.cfg.stack_addr,
                    stack_size + 0x2000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create stack map");
        }

        self.regs_aarch64_mut().sp = sp;
    }

    pub fn init_stack64_tests(&mut self) {
        self.regs_mut().rsp = 0x000000000014F4B0;
        self.regs_mut().rbp = 0x0000000000000000;
        let stack = self.maps.get_mem_mut("stack");
        stack.set_base(0x0000000000149000);
        stack.set_size(0x0000000000007000);
    }

    //TODO: tests only in tests.rs
    pub fn init_regs_tests(&mut self) {
        self.regs_mut().rax = 0x00000001448A76A4;
        self.regs_mut().rbx = 0x000000007FFE0385;
        self.regs_mut().rcx = 0x0000000140000000;
        self.regs_mut().rdx = 0x0000000000000001;
        self.regs_mut().rsi = 0x0000000000000001;
        self.regs_mut().rdi = 0x000000007FFE0384;
        self.regs_mut().r10 = 0x000000007FFE0384;
        self.regs_mut().r11 = 0x0000000000000246;
        self.regs_mut().r12 = 0x00000001448A76A4;
        self.regs_mut().r14 = 0x0000000140000000;
    }

    //TODO: tests only in tests.rs
    pub fn init_flags_tests(&mut self) {
        self.flags_mut().clear();

        self.flags_mut().f_zf = true;
        self.flags_mut().f_pf = true;
        self.flags_mut().f_af = false;

        self.flags_mut().f_of = false;
        self.flags_mut().f_sf = false;
        self.flags_mut().f_df = false;

        self.flags_mut().f_cf = false;
        self.flags_mut().f_tf = false;
        self.flags_mut().f_if = true;

        self.flags_mut().f_nt = false;
    }

    pub fn init_logger(&mut self) {
        fast_log::init(
            fast_log::Config::new()
                .format(CustomLogFormat::new())
                .console()
                .chan_len(Some(100000)),
        )
        .unwrap();
    }

    /// Initialize windows simulator, this does like init_cpu() but also setup the windows memory.
    /// This require having the map files in place, otherwise use just init_cpu() but emu32() and
    /// emu64() already call init_cpu()
    /// This is called from load_code if the code is a PE or shellcode.
    /// load_code_bytes() and other loading ways don't call this, if you need windows simulation call this.
    pub fn init_win32(&mut self, clear_registers: bool, clear_flags: bool) {
        self.pos = 0;

        if !atty::is(Stream::Stdout) {
            self.cfg.nocolors = true;
            self.colors.disable();
            // Auto-disable the interactive console only when the user did NOT
            // explicitly ask for it (`--handle` sets `enabled_ctrlc`, `-c`/`-C`
            // set `console_enabled`). An explicit request must work even with
            // redirected/piped stdout.
            if !self.enabled_ctrlc && !self.cfg.console_enabled {
                self.cfg.console_enabled = false;
                self.disable_ctrlc();
            }
        }

        // Ensure arch_state and thread context match the target architecture before
        // touching any registers, since regs_mut()/regs_aarch64_mut() panic on mismatch.
        if self.cfg.is_aarch64() {
            if matches!(self.arch_state, super::ArchState::X86 { .. }) {
                self.arch_state = super::ArchState::AArch64 {
                    instruction: None,
                    instruction_cache: crate::emu::disassemble::InstructionCache::new(),
                };
            }
            if matches!(
                self.threads[self.current_thread_id].arch,
                crate::threading::context::ArchThreadState::X86 { .. }
            ) {
                let id = self.threads[self.current_thread_id].id;
                self.threads[self.current_thread_id] =
                    crate::threading::context::ThreadContext::new(id, self.cfg.arch);
            }
        }

        //log::trace!("initializing regs");
        if self.cfg.is_aarch64() {
            // AArch64: zero all registers; no x86 flags to clear.
            if clear_registers {
                *self.regs_aarch64_mut() = crate::arch::aarch64::regs::RegsAarch64::new();
            }
            self.regs_aarch64_mut().pc = self.cfg.entry_point;
        } else {
            if clear_registers {
                self.regs_mut().clear::<64>();
            }
            if clear_flags {
                self.flags_mut().clear();
            }
            self.regs_mut().rip = self.cfg.entry_point;
        }
        if self.cfg.arch.is_64bits() {
            self.maps.is_64bits = true;
            if self.cfg.is_aarch64() {
                self.init_win_aarch64();
            } else {
                self.init_win32_mem64();
                self.init_stack64();
            }
        } else {
            // 32bits
            self.maps.is_64bits = false;
            self.regs_mut().sanitize32();
            self.init_win32_mem32();
            self.init_stack32();
        }

        // loading banzai on 32bits
        if !self.cfg.arch.is_64bits() {
            let mut rdr = ReaderBuilder::new()
                .from_path(format!("{}/banzai.csv", self.cfg.maps_folder))
                .expect("banzai.csv not found on maps folder, please download last mwemu maps");

            for result in rdr.records() {
                let record = result.expect("error parsing banzai.csv");
                let api = &record[0];
                let params: i32 = record[1].parse().expect("error parsing banzai.csv");

                self.banzai.add(api, params);
            }
        }

        //self.init_tests();
    }

    /// Return a mutable reference to the managed heap, creating it on demand.
    ///
    /// `heap_management` is only set up eagerly by the 64-bit Windows and Linux
    /// init paths. The 32-bit Windows path (`init_win32_mem32`), the SSDT/syscall
    /// mode (which returns early before the heap block), and a freshly
    /// deserialized emulator all leave it `None`. Without this, the first
    /// `HeapAlloc`/`LocalAlloc` of a small block panicked on `unwrap()`.
    /// Lazily reserve a 4 MiB arena the first time it's needed so every path
    /// has a working heap.
    pub fn heap_mut(&mut self) -> &mut O1Heap {
        if self.heap_management.is_none() {
            let heap_sz: u64 = 4 * 1024 * 1024; // 4 MiB
            let base = self.maps.alloc(heap_sz).expect("cannot reserve heap arena");
            self.maps
                .create_map(".heap", base, heap_sz, Permission::READ_WRITE)
                .expect("cannot create heap map");
            if self.heap_addr == 0 {
                self.heap_addr = base;
            }
            self.heap_management = Some(Box::new(
                O1Heap::new(base, heap_sz as u32).expect("cannot init heap management"),
            ));
        }
        self.heap_management.as_mut().unwrap()
    }

    /// The minimum initializations necessary to emualte asm with no OS simulation.
    pub fn init_cpu(&mut self) {
        self.pos = 0;
        self.maps.is_64bits = self.cfg.arch.is_64bits();

        // Ensure thread context matches the target architecture
        if self.cfg.arch.is_aarch64()
            && matches!(
                self.threads[self.current_thread_id].arch,
                crate::threading::context::ArchThreadState::X86 { .. }
            ) {
                let id = self.threads[self.current_thread_id].id;
                self.threads[self.current_thread_id] =
                    crate::threading::context::ThreadContext::new(id, self.cfg.arch);
            }

        // Ensure arch_state matches the target architecture
        if self.cfg.arch.is_aarch64() && matches!(self.arch_state, super::ArchState::X86 { .. }) {
            self.arch_state = super::ArchState::AArch64 {
                instruction: None,
                instruction_cache: crate::emu::disassemble::InstructionCache::new(),
            };
        }

        if self.cfg.arch.is_aarch64() {
            self.init_stack_aarch64();
        } else if self.cfg.is_x64() {
            self.init_stack64();
        } else {
            self.regs_mut().sanitize32();
            self.init_stack32()
        }
    }

    /// Initialize linux aarch64 simulation for ELF loading.
    pub fn init_linux64_aarch64(&mut self) {
        // Ensure thread context is aarch64
        if matches!(
            self.threads[self.current_thread_id].arch,
            crate::threading::context::ArchThreadState::X86 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] =
                crate::threading::context::ThreadContext::new(id, self.cfg.arch);
        }

        self.ensure_arch_state_aarch64();
        self.init_stack_aarch64();
    }

    /// Switch `Emu::arch_state` to the AArch64 variant if it isn't already.
    /// Required when a binary's loader bumps `cfg.arch` to AArch64 at load
    /// time (e.g. Mach-O / ELF auto-detection from a CLI run that defaulted
    /// to x86): without this the run loop sees `cfg.arch.is_aarch64() ==
    /// true` but `arch_state == ArchState::X86 {..}` and panics with
    /// `unreachable!()` on its first decode iteration.
    pub fn ensure_arch_state_aarch64(&mut self) {
        if matches!(self.arch_state, crate::emu::ArchState::AArch64 { .. }) {
            return;
        }
        self.arch_state = crate::emu::ArchState::AArch64 {
            instruction: None,
            instruction_cache: crate::emu::disassemble::InstructionCache::new(),
        };
    }

    /// Write the Linux initial stack layout that _start expects.
    ///
    /// The kernel places this on the stack before jumping to the ELF entry point:
    ///   [SP+0]  argc
    ///   [SP+8]  argv[0]  (pointer to program name)
    ///   [SP+16] NULL     (argv terminator)
    ///   [SP+24] NULL     (envp terminator)
    ///   auxv[]: AT_PHDR, AT_PHENT, AT_PHNUM, AT_PAGESZ, AT_ENTRY, AT_NULL
    pub fn write_linux_stack_layout(
        &mut self,
        entry: u64,
        phdr_addr: u64,
        phentsize: u16,
        phnum: u16,
    ) {
        let sp = if self.cfg.arch.is_aarch64() {
            self.regs_aarch64().sp
        } else {
            self.regs().rsp
        };

        // Write a program name string above the stack layout area
        let prog_name_addr = sp + 0x100;
        let prog_name = b"prog\0";
        for (i, &b) in prog_name.iter().enumerate() {
            self.maps.write_byte(prog_name_addr + i as u64, b);
        }

        let mut off = sp;

        // argc = 1
        self.maps.write_qword(off, 1);
        off += 8;

        // argv[0] = pointer to program name
        self.maps.write_qword(off, prog_name_addr);
        off += 8;

        // argv terminator
        self.maps.write_qword(off, 0);
        off += 8;

        // envp terminator
        self.maps.write_qword(off, 0);
        off += 8;

        // Auxiliary vector entries (each is 16 bytes: type + value)
        const AT_PHDR: u64 = 3;
        const AT_PHENT: u64 = 4;
        const AT_PHNUM: u64 = 5;
        const AT_PAGESZ: u64 = 6;
        const AT_ENTRY: u64 = 9;
        const AT_NULL: u64 = 0;

        let auxv: &[(u64, u64)] = &[
            (AT_PAGESZ, 4096),
            (AT_PHDR, phdr_addr),
            (AT_PHENT, phentsize as u64),
            (AT_PHNUM, phnum as u64),
            (AT_ENTRY, entry),
            (AT_NULL, 0),
        ];

        for &(atype, aval) in auxv {
            self.maps.write_qword(off, atype);
            off += 8;
            self.maps.write_qword(off, aval);
            off += 8;
        }
    }

    /// Initialize macOS aarch64 simulation for Mach-O loading.
    pub fn init_macos_aarch64(&mut self) {
        self.os = crate::arch::OperatingSystem::MacOS;

        // Ensure thread context is aarch64
        if matches!(
            self.threads[self.current_thread_id].arch,
            crate::threading::context::ArchThreadState::X86 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] =
                crate::threading::context::ThreadContext::new(id, self.cfg.arch);
        }

        self.init_stack_aarch64();
    }

    /// Initialize macOS x86_64 simulation for Mach-O loading.
    pub fn init_macos64(&mut self) {
        self.os = crate::arch::OperatingSystem::MacOS;
        self.maps.is_64bits = true;

        // Mach-O front-door loading can switch the emulator from an earlier
        // AArch64 session, so restore the x86 thread/register state explicitly.
        if matches!(
            self.threads[self.current_thread_id].arch,
            crate::threading::context::ArchThreadState::AArch64 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] =
                crate::threading::context::ThreadContext::new(id, self.cfg.arch);
        }

        if matches!(self.arch_state, super::ArchState::AArch64 { .. }) {
            let mut formatter = IntelFormatter::new();
            formatter.options_mut().set_digit_separator("");
            formatter.options_mut().set_first_operand_char_index(6);
            self.arch_state = super::ArchState::X86 {
                instruction: None,
                formatter,
                instruction_cache: InstructionCache::new(),
                decoder_position: 0,
            };
        }

        self.flags_mut().clear();
        self.flags_mut().f_if = true;
        self.init_stack64();
    }

    /// Initialize linux x86_64 simulation, it's called from load_code() if the sample is an ELF.
    pub fn init_linux64(&mut self, dyn_link: bool) {
        //self.regs_mut().clear::<64>();
        self.flags_mut().clear();
        self.flags_mut().f_if = true;

        // Avoid `set_current_dir` here: the process-wide CWD races with any
        // sibling test running in parallel (cargo test uses many threads) —
        // most visibly causing the Win64 auto-detect in `init_win32_mem64`
        // to miss `maps/windows/x86_64`. Build absolute paths instead.
        if dyn_link {
            //self.regs_mut().rsp = 0x7fffffffe2b0;
            self.regs_mut().rsp = 0x7fffffffe790;
            self.maps
                .create_map(
                    "linux_dynamic_stack",
                    0x7ffffffde000,
                    0x100000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create linux_dynamic_stack map");
            //self.maps.create_map("dso_dyn").load_at(0x7ffff7ffd0000);
            self.maps
                .create_map("dso_dyn", 0x7ffff7ffd000, 0x1000, Permission::READ_WRITE)
                .expect("cannot create dso_dyn map");
            self.maps
                .create_map(
                    "linker",
                    0x7ffff7ffd000 - 0x1000 - 0x10000,
                    0x10000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create linker map");
        } else {
            self.regs_mut().rsp = 0x7fffffffe270;
            self.maps
                .create_map(
                    "linux_static_stack",
                    0x7ffffffde000,
                    0x100000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create linux_static_stack map");
            self.maps
                .create_map("dso", 0x7ffff7ffd000, 0x100000, Permission::READ_WRITE)
                .expect("cannot create dso map");
        }
        let tls = self
            .maps
            .create_map("tls", 0x7ffff8fff000, 0xfff, Permission::READ_WRITE)
            .expect("cannot create tls map");
        // Resolve `tls.bin` against `cfg.maps_folder` so we don't depend on
        // the process-wide CWD (which a parallel test may have changed).
        let tls_path = self.cfg.get_maps_folder("tls.bin");
        tls.load(&tls_path);

        if dyn_link {
            //heap.set_base(0x555555579000);
        } else {
            // here we are allocating 4MB of heap memory
            let heap_sz = 0x885900 - 0x4b5000;
            self.heap_addr = self.maps.alloc(heap_sz).expect("cannot allocate heap");
            let heap = self
                .maps
                .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE) //.create_map("heap", 0x4b5b00, 0x4d8000 - 0x4b5000)
                .expect("cannot create heap map");
            heap.load("heap.bin");

            self.heap_management = Some(Box::new(
                O1Heap::new(self.heap_addr, heap_sz as u32)
                    .expect("Expect new heap_management but failed"),
            ));
        }

        self.regs_mut().rbp = 0;

        self.fs_mut().insert(0xffffffffffffffc8, 0); //0x4b6c50
        self.fs_mut().insert(0xffffffffffffffd0, 0);
        self.fs_mut().insert(0xffffffffffffffd8, 0x4b27a0);
        self.fs_mut().insert(0xffffffffffffffa0, 0x4b3980);
        self.fs_mut().insert(0x18, 0);
        self.fs_mut().insert(40, 0x4b27a0);
    }

    /// This is called from init(), this setup the 32bits windows memory simulation.
    pub fn init_win32_mem32(&mut self) {
        log::trace!("loading memory maps");

        self.maps.is_64bits = self.cfg.arch.is_64bits();

        // (Historic `set_current_dir(maps_folder)` removed — the block in
        // between had no file-system calls and the chdir raced with parallel
        // tests that needed the process CWD to point at the repo root.)

        peb32::init_peb(self);
        winapi32::kernel32::load_library(self, "ntdll.dll");
        let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();
        peb32::update_peb_image_base(self, ntdll_base as u32);

        winapi32::kernel32::load_library(self, "kernel32.dll");
        winapi32::kernel32::load_library(self, "kernelbase.dll");
        winapi32::kernel32::load_library(self, "iphlpapi.dll");
        winapi32::kernel32::load_library(self, "ws2_32.dll");
        winapi32::kernel32::load_library(self, "advapi32.dll");
        //winapi32::kernel32::load_library(self, "comctl32.dll");
        winapi32::kernel32::load_library(self, "winhttp.dll");
        winapi32::kernel32::load_library(self, "wininet.dll");
        //winapi32::kernel32::load_library(self, "dnsapi.dll");
        winapi32::kernel32::load_library(self, "shell32.dll");
        //winapi32::kernel32::load_library(self, "shlwapi.dll");

        let teb_map = self.maps.get_mem_mut("teb");
        let mut teb = structures::TEB::load_map(teb_map.get_base(), teb_map);
        teb.nt_tib.stack_base = self.cfg.stack_addr as u32;
        teb.nt_tib.stack_limit = (self.cfg.stack_addr + 0x30000) as u32;
        teb.save(teb_map);
    }

    //TODO: tests on tests.rs
    pub fn init_tests(&mut self) {
        let mem = self
            .maps
            .create_map("test", 0, 1024, Permission::READ_WRITE_EXECUTE)
            .expect("cannot create test map");
        mem.write_qword(0, 0x1122334455667788);
        assert!(mem.read_qword(0) == 0x1122334455667788);
        self.maps.free("test");

        // some tests
        assert!(get_bit!(0xffffff00u32, 0) == 0);
        assert!(get_bit!(0xffffffffu32, 5) == 1);
        assert!(get_bit!(0xffffff00u32, 5) == 0);
        assert!(get_bit!(0xffffff00u32, 7) == 0);
        assert!(get_bit!(0xffffff00u32, 8) == 1);

        let mut a: u32 = 0xffffff00;
        set_bit!(a, 0, 1);
        set_bit!(a, 1, 1);
        set_bit!(a, 2, 1);
        set_bit!(a, 3, 1);
        set_bit!(a, 4, 1);
        set_bit!(a, 5, 1);
        set_bit!(a, 6, 1);
        set_bit!(a, 7, 1);

        assert!(a == 0xffffffff);

        set_bit!(a, 0, 0);
        set_bit!(a, 1, 0);
        set_bit!(a, 2, 0);
        set_bit!(a, 3, 0);
        set_bit!(a, 4, 0);
        set_bit!(a, 5, 0);
        set_bit!(a, 6, 0);
        set_bit!(a, 7, 0);

        assert!(a == 0xffffff00);

        /*
        remove this test because it isn't that correct
        let mut r: u64;
        (r, _) = engine::logic::shrd(self, 0x9fd88893, 0x1b, 0x6, 32);
        assert!(r == 0x6e7f6222);
        (r, _) = engine::logic::shrd(self, 0x6fdcb03, 0x0, 0x6, 32);
        assert!(r == 0x1bf72c);
        (r, _) = engine::logic::shrd(self, 0x91545f1d, 0x6fe2, 0x6, 32);
        assert!(r == 0x8a45517c);
        (r, _) = engine::logic::shld(self, 0x1b, 0xf1a7eb1d, 0xa, 32);
        assert!(r == 0x6fc6);
        (r, _) = engine::logic::shld(self, 0x1, 0xffffffff, 4, 32);
        assert!(r == 0x1f);
        (r, _) = engine::logic::shld(self, 0x1, 0xffffffff, 33, 32);
        assert!(r == 0x3);
        (r, _) = engine::logic::shld(self, 0x144e471f8, 0x14F498, 0x3e, 64);
        assert!(r == 0x53d26);
        */

        assert!(self.maps.mem_test(), "It doesn't pass the memory tests!!");
        log::trace!("memory test Ok.");
    }

    /// Initialize Windows ARM64 session: sets up aarch64 stack and full PEB64/TEB64/LDR64
    /// memory layout. Arch state and thread context must already be switched to AArch64
    /// before calling this (done at the top of init_win32).
    ///
    /// Reuses the same 64-bit Windows memory setup as x86_64 since PEB64/TEB64/LDR
    /// structures are architecture-neutral; the difference is register semantics and stack init.
    fn init_win_aarch64(&mut self) {
        // Set up the aarch64 stack first (uses SP register instead of RSP/RBP)
        // so that the stack map exists when init_win32_mem64 writes TEB stack bounds.
        self.init_stack_aarch64();

        // Set up the 64-bit Windows memory (PEB64, TEB64, LDR, DLLs, heap)
        self.init_win32_mem64();
    }

    /// This is called from init(), this setup the 64bits windows memory simulation.
    pub fn init_win32_mem64(&mut self) {
        log::trace!("loading memory maps");
        self.maps.is_64bits = self.cfg.arch.is_64bits();

        // In SSDT mode we can optionally let `ntdll!LdrInitializeThunk` bootstrap the loader.
        // This path intentionally maps far fewer DLLs up front.
        if self.cfg.emulate_winapi {
            // Make sure cfg.exe_name reflects the real image basename BEFORE
            // PEB/LDR init writes it into the canonical structures.
            peb64::refresh_exe_name_from_filename(self);
            // Empty PEB + TEB only; `ntdll!LdrInitializeThunk` is expected to initialize loader state.
            peb64::init_peb_teb_empty(self);
            kuser_shared::init_kuser_shared_data(self);

            // ntdll must be pre-mapped so we can resolve and call
            // `LdrInitializeThunk`.
            winapi64::kernel32::load_library(self, "ntdll.dll");

            // Build the syscall number translation table from this ntdll's
            // actual syscall stubs. Microsoft renumbers syscalls between
            // builds (Win10 vs Win11/Server2022 are notably different), and
            // our dispatcher matches on hard-coded constants tied to one
            // build. Without this step a Win2022 ntdll calling syscall 0x16d
            // (NtRaiseException) lands in our 0x16d handler (was the older
            // NtQuerySystemEnvironmentValueEx) and silently misroutes —
            // RtlRaiseException then returns "0" instead of dispatching the
            // exception, leading to a RtlRaiseStatus(0) recursion.
            crate::syscall::windows::syscall64::build_syscall_translation_table(self);

            // Pre-map kernel32 and kernelbase even though modern ntdll
            // clears the legacy `InMemoryOrderModuleList` during init —
            // we don't pre-load them for ordering, but so their `.pe`
            // maps exist when we re-populate the linked list afterwards
            // (LdrInit on Win10+ relies on syscalls we don't fully model,
            // and bails before bringing these in via the KnownDll path).
            //winapi64::kernel32::load_library(self, "kernel32.dll");
            //winapi64::kernel32::load_library(self, "kernelbase.dll");

            // ntdll patches its own globals during LdrInitializeThunk (LdrpHashTable,
            // LdrpModuleBaseAddressIndex, etc.).  These often fall in the .rdata section
            // which our PE loader maps read-only; add write permission to all ntdll maps
            // so the self-patching writes succeed.
            let ntdll_map_names: Vec<String> = self
                .maps
                .name_map
                .keys()
                .filter(|n| n.starts_with("ntdll"))
                .cloned()
                .collect();
            for name in ntdll_map_names {
                if let Some(mem) = self.maps.get_map_by_name_mut(&name) {
                    mem.add_permission(crate::maps::mem64::Permission::WRITE);
                }
            }

            // Patch ntdll's LdrpHandleInvalidUserCallTarget / RtlFailFast2 terminal block.
            // The sequence `mov edx, 0xc0000409; mov rcx, -1; call NtTerminateProcess` triggers
            // during LdrInitializeThunk for CFG violations in our emulated environment.
            // Replace the `mov edx` with a `jmp` to the `ret` that follows the call, so the
            // handler returns harmlessly instead of terminating the process.
            // Pattern: ba 09 04 00 c0  48 c7 c1 ff ff ff ff  e8 ?? ?? ?? ??  [??x7]  c3
            // (5 bytes mov edx) + (7 bytes mov rcx,-1) + (5 bytes call) + (7 bytes add rsp) + ret
            if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                let ntdll_base = ntdll_pe.get_base();
                // ntdll.pe is just the PE header — scan the full image (≤ 8 MiB covers all sections).
                let ntdll_size: usize = 0x800000;
                // Search for the pattern in the ntdll image.
                // We look for mov-edx + mov-rcx(-1) = ba 09 04 00 c0  48 c7 c1 ff ff ff ff
                let needle: &[u8] = &[
                    0xba, 0x09, 0x04, 0x00, 0xc0, 0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff,
                ];
                let mut found_va: Option<u64> = None;
                for off in 0..ntdll_size.saturating_sub(needle.len() + 10) {
                    let va = ntdll_base + off as u64;
                    let mut matches = true;
                    for (i, &b) in needle.iter().enumerate() {
                        if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                            matches = false;
                            break;
                        }
                    }
                    if matches {
                        // Verify there is a `ret` (c3) 24 bytes after pattern start:
                        // needle(12) + call(5) + add rsp,0x88(7) = 24.
                        let ret_off = needle.len() as u64 + 5 + 7; // = 24
                        if self.maps.read_byte(va + ret_off).unwrap_or(0) == 0xc3 {
                            found_va = Some(va);
                            break;
                        }
                    }
                }
                if let Some(patch_va) = found_va {
                    // Layout from patch_va:
                    //  +0x00 mov edx,0xc0000409  (5 bytes)  ← replaced with jmp rel32
                    //  +0x05 mov rcx,-1           (7 bytes)
                    //  +0x0c call NtTerminateProcess (5 bytes)
                    //  +0x11 add rsp,0x88          (7 bytes)  ← jump TARGET (must not skip this)
                    //  +0x18 ret
                    // displacement = 0x11 - 5 = 0x0C
                    let _ = self.maps.write_byte(patch_va, 0xe9); // jmp rel32
                    let _ = self.maps.write_byte(patch_va + 1, 0x0c); // disp: jump to add rsp,0x88
                    let _ = self.maps.write_byte(patch_va + 2, 0x00);
                    let _ = self.maps.write_byte(patch_va + 3, 0x00);
                    let _ = self.maps.write_byte(patch_va + 4, 0x00);
                    log::debug!("ntdll CFG RtlFailFast2 patch applied at 0x{:x}", patch_va);
                } else {
                    log::debug!("ntdll CFG RtlFailFast2 pattern not found — skipping patch");
                }

                // LdrpInitSecurityCookie double-init patch
                // ---------------------------------------------------------------
                // ntdll's per-DLL security-cookie randomiser does:
                //   mov r12, 0x2b992ddfa232  ; uninitialised "magic"
                //   cmp [rdi], r12
                //   jne <fail>               ; not magic → STATUS_INVALID_IMAGE_FORMAT
                //   ...randomise rbx...
                //   mov [rdi], rbx           ; store new cookie
                //   mov eax, 1; ret
                //
                // The loader on this ntdll build invokes the routine twice for
                // kernelbase during LdrInitializeThunk; the second call finds
                // the cookie already randomised and trips the `jne` → process
                // aborts with `STATUS_INVALID_IMAGE_FORMAT (0xC000007B)`,
                // killing every subsequent `LoadLibraryA`.
                //
                // Patch the cookie-store `mov [rdi], rbx` (48 89 1f) to three
                // NOPs.  Effect: the cookie keeps its on-disk "magic" forever.
                // First and every later call see magic, pass the `cmp`, and
                // return success without mutating the page. ntdll continues
                // unaware; stack-canary checks all run against the same magic
                // value, which is fine for emulation.
                if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                    let ntdll_base = ntdll_pe.get_base();
                    let ntdll_size: usize = 0x800000;
                    // Pattern: movabs r12,0x2b992ddfa232 (49 bc 32 a2 df 2d 99 2b 00 00)
                    //          cmp [rdi], r12          (4c 39 27)
                    //          jne short ??            (75 ??)
                    //          mov rcx, rdi            (48 8b cf)
                    //          call ??                 (e8 ?? ?? ?? ??)
                    // followed downstream by the `mov [rdi], rbx` (48 89 1f).
                    let needle: &[u8] = &[
                        0x49, 0xbc, 0x32, 0xa2, 0xdf, 0x2d, 0x99, 0x2b, 0x00, 0x00, 0x4c, 0x39,
                        0x27,
                    ];
                    let mut anchor_va: Option<u64> = None;
                    for off in 0..ntdll_size.saturating_sub(needle.len() + 0x80) {
                        let va = ntdll_base + off as u64;
                        let mut matches = true;
                        for (i, &b) in needle.iter().enumerate() {
                            if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            anchor_va = Some(va);
                            break;
                        }
                    }
                    if let Some(anchor) = anchor_va {
                        // Walk forward looking for `48 89 1f` (mov [rdi], rbx)
                        // within ~0x80 bytes of the cmp.
                        let mut store_va: Option<u64> = None;
                        for d in 0..0x80u64 {
                            let p = anchor + needle.len() as u64 + d;
                            if self.maps.read_byte(p).unwrap_or(0) == 0x48
                                && self.maps.read_byte(p + 1).unwrap_or(0) == 0x89
                                && self.maps.read_byte(p + 2).unwrap_or(0) == 0x1f
                            {
                                store_va = Some(p);
                                break;
                            }
                        }
                        if let Some(p) = store_va {
                            let _ = self.maps.write_byte(p, 0x90);
                            let _ = self.maps.write_byte(p + 1, 0x90);
                            let _ = self.maps.write_byte(p + 2, 0x90);
                            log::debug!(
                                "ntdll LdrpInitSecurityCookie store-cookie nop patch at 0x{:x}",
                                p
                            );
                        } else {
                            log::debug!(
                                "ntdll LdrpInitSecurityCookie cookie-store not found near 0x{:x}",
                                anchor
                            );
                        }
                    } else {
                        log::debug!(
                            "ntdll LdrpInitSecurityCookie cmp pattern not found — skipping patch"
                        );
                    }
                }

                // LdrInitializeThunk error-status bypass patch
                // ---------------------------------------------------------------
                // After `LdrpInitializeProcess` returns, LdrInitializeThunk
                // checks the status with:
                //   41 bc 00 20 00 00   mov  r12d, 0x2000
                //   85 ff               test edi, edi
                //   0f 88 .. .. .. ..   js   <fatal-error path>
                //
                // The fatal path calls `LdrpLogFatalUserError` (→
                // `NtRaiseHardError(STATUS_APP_INIT_FAILURE)`) and then
                // `NtTerminateProcess(edi)`. In Win2022 emulation,
                // LdrpInitializeProcess fails inside `LdrpResGetMappingSize`
                // (resource-section processing of a KnownDll) — a path we
                // don't fully model — and returns STATUS_INTERNAL_ERROR.
                // The rest of the loader state is already populated, so the
                // failure isn't actually fatal for the EXE we want to run.
                //
                // Replace the 6-byte `js rel32` with 6 NOPs so the test
                // result is ignored. Execution falls through to the normal
                // post-LdrInit path that does `test [r14+0x17ee], r12w;
                // call ZwTestAlert; add rsp, 0x40; ret` — leaving the
                // partially-initialised loader state in place for the EXE
                // entrypoint. (Combined with the AC-bypass patch above this
                // gets us all the way past LdrInit on Win2022 ntdll.)
                if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                    let ntdll_base = ntdll_pe.get_base();
                    let ntdll_size: usize = 0x800000;
                    // Pattern: `mov r12d, 0x2000 ; test edi, edi ; js rel32`
                    let needle: &[u8] =
                        &[0x41, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x85, 0xff, 0x0f, 0x88];
                    let mut found_va: Option<u64> = None;
                    for off in 0..ntdll_size.saturating_sub(needle.len() + 4) {
                        let va = ntdll_base + off as u64;
                        let mut matches = true;
                        for (i, &b) in needle.iter().enumerate() {
                            if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            found_va = Some(va);
                            break;
                        }
                    }
                    if let Some(anchor) = found_va {
                        // The 6-byte `js rel32` starts at anchor + 8.
                        let js_va = anchor + 8;
                        for i in 0..6 {
                            let _ = self.maps.write_byte(js_va + i, 0x90);
                        }
                        log::debug!(
                            "ntdll LdrInitializeThunk status-check bypass patch applied at 0x{:x}",
                            js_va,
                        );
                    } else {
                        log::debug!(
                            "ntdll LdrInitializeThunk status-check pattern not found — skipping patch"
                        );
                    }
                }

                // RtlpGetSystemDefaultUILanguage AC-init-call bypass patch
                // ---------------------------------------------------------------
                // Win2022 `ntdll!RtlpGetSystemDefaultUILanguage` (the function
                // body around RVA 0x3b4a0) reaches an internal helper whose
                // job is to write into an activation-context-derived struct.
                // The relevant fragment is:
                //
                //   83 79 38 00       cmp dword ptr [rcx + 0x38], 0
                //   48 8b cb          mov rcx, rbx
                //   74 1a             je  skip                    ; ← patched
                //   e8 .. .. .. ..    call <internal AC helper>   ; faults
                //
                // The helper reads `arg->[0x38]->[0x30]` as a base pointer and
                // adds `arg->[0x84] * 8` as an index, writing one qword at the
                // computed address. Under emulation we don't initialise the
                // activation-context table the helper expects, so the index
                // ends up landing inside `kernelbase.rdata` (read-only) and
                // the write faults — taking the whole `LdrInitializeThunk`
                // down with it (now that the no-handler exception path stops
                // emulation).
                //
                // The branch ABOVE this site (`je 0x18003b4f8`) is the normal
                // "AC table absent → skip helper" path on real Windows; we
                // flip it to an unconditional `jmp` so the call is *always*
                // skipped. Downstream code reads the function's return value
                // as an NTSTATUS, but the success path in the caller already
                // jumps to the same continuation when the helper is skipped,
                // so bypassing it leaves the function semantically equivalent
                // to "no activation context applied" — which matches what
                // mwemu actually models.
                //
                // Pattern intentionally omits the rel8 displacement of the
                // `je` (the last byte): the offset varies between builds, but
                // the cmp/mov-rcx-rbx/je-against-zero sequence is unique.
                if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                    let ntdll_base = ntdll_pe.get_base();
                    let ntdll_size: usize = 0x800000;
                    let needle: &[u8] = &[0x83, 0x79, 0x38, 0x00, 0x48, 0x8b, 0xcb, 0x74];
                    let mut found_va: Option<u64> = None;
                    for off in 0..ntdll_size.saturating_sub(needle.len() + 1) {
                        let va = ntdll_base + off as u64;
                        let mut matches = true;
                        for (i, &b) in needle.iter().enumerate() {
                            if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            found_va = Some(va);
                            break;
                        }
                    }
                    if let Some(anchor) = found_va {
                        // `je` opcode lives at anchor + 7; rewrite as `jmp short`.
                        let je_va = anchor + 7;
                        let _ = self.maps.write_byte(je_va, 0xeb);
                        log::debug!(
                            "ntdll RtlpGetSystemDefaultUILanguage AC bypass patch applied at 0x{:x}",
                            je_va,
                        );
                    } else {
                        log::debug!(
                            "ntdll RtlpGetSystemDefaultUILanguage AC bypass pattern not found — skipping patch"
                        );
                    }
                }
            }

            // Minimal TEB stack bounds (NtTib) so stack probes do not fault.
            // NtTib.StackBase (off +0x08) = HIGH address — the address where RSP
            //   was at thread creation (the "top of stack"). On Windows this is
            //   *not* the end of the reservation but the initial RSP value.
            // NtTib.StackLimit (off +0x10) = LOW address (bottom of committed region).
            //
            // We previously used `map.get_bottom()` for StackBase, which is the end
            // of the underlying mwemu map and sits 0x2000 above the initial RSP.
            // ntdll's exception dispatcher uses NtTib.StackBase as the upper bound
            // for stack scratch buffers; if it's past the real stack top, those
            // writes spill past the map and fault. Use the initial RSP instead.
            let stack_lo = self
                .maps
                .get_map_by_name("stack")
                .map(|s| s.get_base())
                .unwrap_or(self.cfg.stack_addr);
            let stack_hi = self.regs().rsp; // initial RSP = canonical NtTib.StackBase
            let teb_map = self.maps.get_mem_mut("teb");
            let teb_addr = teb_map.get_base();
            let mut teb = structures::TEB64::load_map(teb_addr, teb_map);
            teb.nt_tib.stack_base = stack_hi;
            teb.nt_tib.stack_limit = stack_lo;
            // NtTib.Self must point to the TEB itself (gs:[0x30] canonical value).
            teb.nt_tib.self_pointer = teb_addr;
            teb.save(teb_map);

            return;
        }

        // Update exe_name from the real EXE basename before PEB init writes it.
        peb64::refresh_exe_name_from_filename(self);
        peb64::init_peb(self);
        kuser_shared::init_kuser_shared_data(self);

        let mut metadata: Vec<Lib> = Vec::new();
        let base: Vec<&str> = vec!["kernelbase.dll", "kernel32.dll", "ntdll.dll"];

        // Stage 1: map kernel32
        for dll in &base {
            self.ensure_maps_dll(dll); // fetch from the symbol server if missing
            let filepath = self.cfg.get_maps_folder(dll);
            log::debug!("mapping base lib64: {}", filepath);
            assert!(
                std::path::Path::new(&filepath).exists(),
                "required base DLL not found: {} (maps_folder={})",
                filepath,
                self.cfg.maps_folder
            );
            let (base, pe64, raw) = self.map_dll_pe64(&filepath);
            let lib = Lib {
                pe64,
                raw,
                base,
                name: dll.to_string(),
            };
            metadata.push(lib);
        }

        // Stage 2: get_dependencies
        let mut dependencies: BTreeSet<String> = BTreeSet::new();
        for dll in metadata.iter_mut() {
            for mut dep in dll.pe64.get_dependencies() {
                dep = dep.to_lowercase();
                if !dep.ends_with(".dll") {
                    dep.push_str(".dll");
                }
                if !base.contains(&dep.as_str()) {
                    dependencies.insert(dep);
                }
            }
        }

        // Stage 3: map dependencies
        for dll in dependencies {
            self.ensure_maps_dll(&dll); // fetch from the symbol server if missing
            let filepath = self.cfg.get_maps_folder(&dll);
            log::debug!("mapping depenency {}", filepath);
            assert!(
                std::path::Path::new(&filepath).exists(),
                "required dependency DLL not found: {} (maps_folder={})",
                filepath,
                self.cfg.maps_folder
            );
            let (base, pe64, raw) = self.map_dll_pe64(&filepath);
            let lib = Lib {
                pe64,
                raw,
                base,
                name: dll.to_string(),
            };
            metadata.push(lib);
        }

        // Stage 3: dynamic linking base + deps
        for dll in &metadata {
            log::debug!("dynamic linking {}", dll.name);
            peb64::dynamic_link_module(dll.base, dll.pe64.get_pe_off(), &dll.name, self);
        }

        // Stage 3: IAT binding for base + deps (relocs already applied in `map_dll_pe64`).
        for dll in metadata.iter_mut() {
            log::debug!("iat binding {}", dll.name);
            dll.pe64.iat_binding(&dll.raw, self, dll.base);
            dll.pe64.delay_load_binding(&dll.raw, self, dll.base);
        }
        log::debug!("win32 64bits base libs ok.");

        let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();
        peb64::update_peb_image_base(self, ntdll_base);

        let (stack_base, stack_limit) = self
            .maps
            .get_map_by_name("stack")
            .map(|s| (s.get_base(), s.get_bottom()))
            .unwrap_or((self.cfg.stack_addr, self.cfg.stack_addr + 0x100000 + 0x2000));
        let teb_map = self.maps.get_mem_mut("teb");
        let mut teb = structures::TEB64::load_map(teb_map.get_base(), teb_map);
        teb.nt_tib.stack_base = stack_base;
        teb.nt_tib.stack_limit = stack_limit;
        teb.save(teb_map);

        let heap_sz = 0x885900 - 0x4b5000;
        self.heap_addr = 0x520000; // Hardcoded in PEB64
        let heap = self
            .maps
            .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE)
            .expect("cannot create heap map");

        // Native ntdll!RtlAllocateHeap expects SegmentSignature at offset 0x10
        self.maps.write_dword(self.heap_addr + 0x10, 0x0DDEEDDEE);

        // ntdll!RtlAllocateHeap accesses FreeLists/BlocksIndex. If 0, it crashes dereferencing NULL.
        // We put a self-referential or valid pointer so it doesn't crash on [r10+2].
        // At 0x5203D8 (rsi+rcx*8+80h) it expects a pointer to something. We point it to 0x520400.
        self.maps
            .write_qword(self.heap_addr + 0x3D8, self.heap_addr + 0x400);

        // Later accesses [0x520480] and passes it as locking structure. Needs to be != 0 to avoid [0x10] unmapped array
        self.maps
            .write_qword(self.heap_addr + 0x480, self.heap_addr + 0x500);

        // At 0x520418 it checks [rdi] == rdi to see if list is empty
        self.maps
            .write_qword(self.heap_addr + 0x418, self.heap_addr + 0x418);

        self.heap_management = Some(Box::new(
            O1Heap::new(self.heap_addr, heap_sz as u32)
                .expect("Expect new heap_management but failed"),
        ));
    }
}
