use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::time::Instant;

use crate::emu::{Emu, InstructionState};
use crate::maps::mem64::Permission;
use crate::{
    api::banzai::Banzai, config::Config, debug::breakpoint::Breakpoints, hooks::Hooks, maps::Maps,
    threading::context::ThreadContext, threading::global_locks::GlobalLocks, utils::colors::Colors,
};

use crate::emu::object_handle::HandleManagement;
use crate::maps::heap_allocation::O1Heap;
use crate::windows::export_index::ExportIndexRegistry;
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

impl Emu {
    /// Create a new emulator for the given architecture.
    pub fn new(arch: crate::arch::Arch) -> Emu {
        let instruction_state = InstructionState::default();
        let mut cfg = Config::new();
        cfg.arch = arch;

        Emu {
            instruction_state,
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
            pe32_flat_loaded: false,
            elf64: None,
            elf32: None,
            macho64: None,
            kernel: None,
            kernel_guard: false,
            memory_operations: vec![],
            rep: None,
            tick: 0,
            trace_file: None,
            api_call_log: std::collections::VecDeque::new(),
            base: 0,
            heap_addr: 0,
            rng: {
                use rand::SeedableRng as _;
                rand::rngs::StdRng::from_os_rng()
            },
            rng_seed: None,
            threads: vec![ThreadContext::new(0x1000, arch)],
            current_thread_id: 0,
            global_locks: GlobalLocks::new(),
            processes: crate::threading::process::ProcessTable::new(),
            definitions: HashMap::new(),
            stored_contexts: HashMap::new(),
            entropy: 0.0,
            heap_arenas: Vec::new(),
            last_error: 0,
            count_read: 0,
            count_write: 0,
            count_send: 0,
            count_recv: 0,
            is_api_run: false,
            ld_bootstrap: false,
            is_break_on_api: false,
            instruction_count: 0,
            fault_count: 0,
            handle_management: HandleManagement::new(),
            library_loaded: false,
            section_handles: HashMap::new(),
            file_section_names: HashMap::new(),
            file_handles: HashMap::new(),
            section_view_dll: HashMap::new(),
            validated_knowndlls: HashSet::new(),
            last_closed_section_dll: None,
            fts_handles: HashMap::new(),
            getopt_char_index: 0,
            emulated_stdout: Vec::new(),
            syscall_number_map: HashMap::new(),
            syscall_name_by_real: HashMap::new(),
            known_dll_dir_handles: HashSet::new(),
            console_handles: HashSet::new(),
            api_resolve_cache: HashMap::new(),
            api_addr_name_cache: HashMap::new(),
            export_indexes: ExportIndexRegistry::new(),
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
            match self.maps.create_map(
                "stack",
                self.cfg.stack_addr,
                0x030000,
                Permission::READ_WRITE,
            ) {
                Ok(stack) => (stack.get_base(), stack.get_bottom()),
                Err(e) => {
                    log::error!("cannot create stack map: {}", e);
                    return;
                }
            }
        };

        if esp >= ebp
            || esp <= stack_base
            || esp >= stack_bottom
            || ebp <= stack_base
            || ebp >= stack_bottom
        {
            log::error!(
                "init_stack32: stack pointer validation failed esp=0x{:x} ebp=0x{:x} base=0x{:x} bottom=0x{:x}",
                esp,
                ebp,
                stack_base,
                stack_bottom
            );
        }
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
            match self.maps.create_map(
                "stack",
                self.cfg.stack_addr,
                stack_size + 0x2000,
                Permission::READ_WRITE,
            ) {
                Ok(stack) => (stack.get_base(), stack.get_bottom()),
                Err(e) => {
                    log::error!("cannot create stack map: {}", e);
                    return;
                }
            }
        };

        if rsp >= rbp
            || rsp <= stack_base
            || rsp >= stack_bottom
            || rbp <= stack_base
            || rbp >= stack_bottom
        {
            log::error!(
                "init_stack64: stack pointer validation failed rsp=0x{:x} rbp=0x{:x} base=0x{:x} bottom=0x{:x}",
                rsp,
                rbp,
                stack_base,
                stack_bottom
            );
        }
    }

    //TODO: tests only in tests.rs
    pub fn init_stack_aarch64(&mut self) {
        let stack_size: u64 = 0x100000;

        if self.cfg.stack_addr == 0 {
            self.cfg.stack_addr = 0x22a000;
        }

        let sp = self.cfg.stack_addr + stack_size;

        if self.maps.get_map_by_name("stack").is_none()
            && let Err(e) = self.maps.create_map(
                "stack",
                self.cfg.stack_addr,
                stack_size + 0x2000,
                Permission::READ_WRITE,
            )
        {
            log::error!("cannot create aarch64 stack map: {}", e);
            return;
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

    /// Return a mutable reference to the managed heap, creating it on demand.
    ///
    /// `heap_arenas` is only set up eagerly by the 64-bit Windows and Linux
    /// init paths. The 32-bit Windows path (`init_win32_mem32`), the SSDT/syscall
    /// mode (which returns early before the heap block), and a freshly
    /// deserialized emulator all leave it empty. Without this, the first
    /// `HeapAlloc`/`LocalAlloc` of a small block panicked on `unwrap()`.
    /// Lazily reserve a 4 MiB arena the first time it's needed so every path
    /// has a working heap. Entry 0 of `heap_arenas` is always the process heap.
    pub fn heap_mut(&mut self) -> &mut O1Heap {
        if self.heap_arenas.is_empty() {
            let heap_sz: u64 = 4 * 1024 * 1024; // 4 MiB
            let base = match self.maps.alloc(heap_sz) {
                Some(b) => b,
                None => {
                    log::error!(
                        "cannot reserve heap arena (alloc returned None), using fallback at 0x10000"
                    );
                    0x10000
                }
            };
            let map_name = if !self.maps.exists_mapname(".heap") {
                ".heap"
            } else {
                ".heap_lazy"
            };
            if let Err(e) = self
                .maps
                .create_map(map_name, base, heap_sz, Permission::READ_WRITE)
            {
                log::error!("cannot create {} map: {}", map_name, e);
            }
            if self.heap_addr == 0 {
                self.heap_addr = base;
            }
            match O1Heap::new(base, heap_sz as u32) {
                Ok(heap) => self.heap_arenas.push(Box::new(heap)),
                Err(e) => {
                    log::error!("cannot init heap management: {}", e);
                    match O1Heap::new(base, O1Heap::MIN_ARENA_SIZE as u32) {
                        Ok(heap) => self.heap_arenas.push(Box::new(heap)),
                        Err(_) => panic!("fatal: cannot initialize any heap arena"),
                    }
                }
            }
        }
        self.heap_arenas.first_mut().unwrap()
    }

    /// Create a fresh private O1Heap arena for `HeapCreate`. Ensures entry 0
    /// (the process heap) exists first so the new arena always lives at
    /// index >= 1. Returns the new arena index, or `None` when the guest
    /// memory could not be reserved (Windows would return a NULL handle).
    pub fn create_heap_arena(
        &mut self,
        initial_size: usize,
        maximum_size: usize,
        opts: u32,
    ) -> Option<usize> {
        let _ = self.heap_mut();
        let default_size = 4 * 1024 * 1024;
        let requested_size = if maximum_size == 0 {
            initial_size.max(default_size)
        } else {
            initial_size.max(maximum_size)
        };
        let arena_size = requested_size
            .max(O1Heap::MIN_ARENA_SIZE)
            .min(u32::MAX as usize);
        let name = format!(".heap_{}", self.heap_arenas.len());

        let base = self.maps.alloc(arena_size as u64)?;
        self.maps
            .create_map(
                name.as_str(),
                base,
                arena_size as u64,
                if opts & 0x40000 != 0 {
                    Permission::READ_WRITE_EXECUTE
                } else {
                    Permission::READ_WRITE
                },
            )
            .ok()?;
        let heap = O1Heap::new(base, arena_size as u32).ok()?;
        self.heap_arenas.push(Box::new(heap));
        Some(self.heap_arenas.len() - 1)
    }

    /// Release the guest memory backing a private heap arena created by
    /// `create_heap_arena`. The `O1Heap` entry stays in `heap_arenas` as an
    /// unreachable tombstone so the indices stored in other `HeapHandle`s
    /// remain valid; its map name is `.heap_{idx}` (arena 0, the process
    /// heap, is `.heap` and must never be destroyed here).
    pub fn destroy_heap_arena(&mut self, arena_idx: usize) {
        if arena_idx == 0 || arena_idx >= self.heap_arenas.len() {
            return;
        }
        let guarded: Vec<_> = self.kernel.as_ref().map_or_else(Vec::new, |kernel| {
            kernel
                .heap
                .chunks()
                .iter()
                .filter(|chunk| chunk.heap_owner == Some(arena_idx) && !chunk.is_freed())
                .map(|chunk| chunk.addr)
                .collect()
        });
        for address in guarded {
            self.kernel_free(address, "HeapDestroy");
        }
        let name = format!(".heap_{}", arena_idx);
        if self.maps.exists_mapname(&name) {
            self.maps.free(&name);
        }
        self.heap_arenas[arena_idx].clear();
    }

    pub fn heap_arena_mut(&mut self, idx: usize) -> Option<&mut O1Heap> {
        if idx == 0 {
            Some(self.heap_mut())
        } else {
            self.heap_arenas
                .get_mut(idx)
                .filter(|arena| self.maps.is_mapped(arena.base()))
                .map(|b| b.as_mut())
        }
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
            )
        {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        // Refresh `instruction_state` so the cached cache stays in sync with `cfg.arch`.
        // The thread context rebuild above already handles the typed payload; we
        // just need to reset the decode cache slot.
        self.instruction_state = InstructionState::default();
        if self.cfg.arch.is_aarch64() {
            self.init_stack_aarch64();
        } else if self.cfg.is_x64() {
            self.init_stack64();
        } else {
            self.regs_mut().sanitize32();
            self.init_stack32()
        }
    }

    /// Reset `Emu::instruction_state` to a fresh AArch64-decoding cache.
    /// Used when loaders bump `cfg.arch` to AArch64 at load time (Mach-O /
    /// ELF auto-detection from an x86 CLI default); without this the run
    /// loop would reuse an x86-formatted state for an AArch64 binary.
    pub fn ensure_arch_state_aarch64(&mut self) {
        if self.cfg.arch.is_aarch64() {
            self.instruction_state = InstructionState::default();
        }
    }

    //TODO: tests on tests.rs
    pub fn init_tests(&mut self) {
        let mem = match self
            .maps
            .create_map("test", 0, 1024, Permission::READ_WRITE_EXECUTE)
        {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create test map: {}", e);
                return;
            }
        };
        mem.write_qword(0, 0x1122334455667788);
        assert_eq!(mem.read_qword(0), 0x1122334455667788);
        self.maps.free("test");

        // some tests
        assert_eq!(get_bit!(0xffffff00u32, 0), 0);
        assert_eq!(get_bit!(0xffffffffu32, 5), 1);
        assert_eq!(get_bit!(0xffffff00u32, 5), 0);
        assert_eq!(get_bit!(0xffffff00u32, 7), 0);
        assert_eq!(get_bit!(0xffffff00u32, 8), 1);

        let mut a: u32 = 0xffffff00;
        set_bit!(a, 0, 1);
        set_bit!(a, 1, 1);
        set_bit!(a, 2, 1);
        set_bit!(a, 3, 1);
        set_bit!(a, 4, 1);
        set_bit!(a, 5, 1);
        set_bit!(a, 6, 1);
        set_bit!(a, 7, 1);

        assert_eq!(a, 0xffffffff);

        set_bit!(a, 0, 0);
        set_bit!(a, 1, 0);
        set_bit!(a, 2, 0);
        set_bit!(a, 3, 0);
        set_bit!(a, 4, 0);
        set_bit!(a, 5, 0);
        set_bit!(a, 6, 0);
        set_bit!(a, 7, 0);

        assert_eq!(a, 0xffffff00);

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
}
