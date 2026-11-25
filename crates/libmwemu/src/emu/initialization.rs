use rand_chacha::ChaCha12Rng;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::IsTerminal;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Once};
use std::time::Instant;

use fast_log::appender::{Command, FastLogRecord, RecordFormat};

use crate::api::unix::ApiState;
use crate::api::windows::export_index::ExportIndexRegistry;
use crate::api::windows::policy::ApiPolicy;
use crate::arch::aarch64::regs::RegsAarch64;
use crate::arch::{Arch, OperatingSystem};
use crate::emu::object_handle::{HandleManagement, HeapHandle};
use crate::emu::{Emu, InstructionState, LastOp};
use crate::err::MwemuError;
use crate::maps::heap_allocation::O1Heap;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, PAGE_SIZE, align_up};
use crate::syscall::linux::GUEST_ID;
use crate::syscall::linux::constants::elf::*;
use crate::syscall::windows::{index_syscall_names, syscall64};
use crate::threading::context::ArchThreadState;
use crate::threading::process::ProcessTable;
use crate::winapi::common::crt_stdio::CrtIoState;
use crate::winapi::common::winsock::sockets::SocketState;
use crate::windows::constants::memory::{DEFAULT_STACK_RESERVE, USER32_ADDRESS_LIMIT};
use crate::windows::peb::{peb32, peb64};
use crate::windows::structures::TEB;
use crate::{
    api::banzai::Banzai, config::Config, debug::breakpoint::Breakpoints, hooks::Hooks, maps::Maps,
    threading::context::ThreadContext, threading::global_locks::GlobalLocks, utils::colors::Colors,
    winapi::winapi32, winapi::winapi64, windows::kuser_shared, windows::structures,
};

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
        Emu::new(Arch::X86)
    }
}

impl Emu {
    /// Create a new emulator for the given architecture.
    pub fn new(arch: Arch) -> Emu {
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
            api_policy: ApiPolicy::default(),
            unix_api: ApiState::default(),
            linux_files: None,
            crt_io: CrtIoState::default(),
            sockets: SocketState::default(),
            oem_file_names: false,
            process_error_mode: 0,
            console_host_process: 0,
            raise_on_invalid_handle_close: false,
            process_dep_flags: 0,
            ipc: Default::default(),
            priorities: Default::default(),
            last_luid: 0x10000,
            atom_tables: Default::default(),
            profiles: Default::default(),
            console_cursor: Default::default(),
            windows_tls_slots: Vec::new(),
            windows_fls_slots: Vec::new(),
            fls_cleanup: Default::default(),
            veh_registration_ids: Vec::new(),
            last_veh_id: 0,
            app_user_model_id: Vec::new(),
            registry: Default::default(),
            crypto: Default::default(),
            runtime_function_tables: Vec::new(),
            guest_windows: Default::default(),
            user_timers: Default::default(),
            wnf_subscriptions: Default::default(),
            gdi_cookie: 0,
            com_apartments: Default::default(),
            keyboard_state: Default::default(),
            environment: None,
            crt_environments: Default::default(),
            process_terminated: false,
            call_depth: 0,
            host_call_return: None,
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
            os: OperatingSystem::Windows,
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
            security_cookie_va: 0,
            last_cmp: LastOp::EMPTY,
            last_flag_op: LastOp::EMPTY,
            last_ret: (0, 0),
            memory_operations: vec![],
            rep: None,
            tick: 0,
            trace_file: None,
            api_call_log: std::collections::VecDeque::new(),
            base: 0,
            heap_addr: 0,
            rng: {
                use rand::SeedableRng as _;
                ChaCha12Rng::from_os_rng()
            },
            rng_seed: None,
            threads: vec![ThreadContext::new(0x1000, arch)],
            current_thread_id: 0,
            global_locks: GlobalLocks::new(),
            processes: ProcessTable::new(),
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
            virtual_system32_dlls: HashSet::new(),
            section_view_dll: HashMap::new(),
            last_closed_section_dll: None,
            fts_handles: HashMap::new(),
            getopt_char_index: 0,
            emulated_stdout: Vec::new(),
            syscall_number_map: HashMap::new(),
            syscall_name_by_real: HashMap::new(),
            known_dll_dir_handles: HashSet::new(),
            api_resolve_cache: HashMap::new(),
            api_addr_name_cache: HashMap::new(),
            export_indexes: ExportIndexRegistry::new(),
            guest_dlls: Default::default(),
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

        self.set_stack32_bounds(stack_bottom, stack_base, stack_base);
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

    fn set_stack32_bounds(&mut self, top: u64, limit: u64, reserve_base: u64) {
        if let Some(teb) = self.windows_teb() {
            self.maps
                .write_dword(teb + TEB::NT_TIB_STACK_BASE_OFFSET, top as u32);
            self.maps
                .write_dword(teb + TEB::NT_TIB_STACK_LIMIT_OFFSET, limit as u32);
            self.maps
                .write_dword(teb + TEB::DEALLOCATION_STACK_OFFSET, reserve_base as u32);
        }
    }

    /// Initialize the process's first thread after the image address and stack sizes are known.
    pub(crate) fn init_pe_stack32(&mut self, reserve: u64, commit: u64) -> Result<(), MwemuError> {
        if self.maps.exists_mapname("stack") {
            // Embedders may supply a live stack before loading the image.
            self.init_stack32();
            return Ok(());
        }
        const DEFAULT_STACK_BASE: u64 = 0x200000;
        const STARTUP_FRAME: u64 = 0x100;

        let commit = align_up(commit.max(PAGE_SIZE), PAGE_SIZE)
            .ok_or_else(|| MwemuError::new("invalid PE32 stack commit"))?;
        let reserve = if reserve == 0 {
            DEFAULT_STACK_RESERVE
        } else {
            reserve
        };
        let minimum = commit
            .checked_add(PAGE_SIZE)
            .ok_or_else(|| MwemuError::new("invalid PE32 stack commit"))?;
        if commit > self.maps.max_alloc_size {
            return Err(MwemuError::new(
                "PE32 stack commit exceeds allocation limit",
            ));
        }
        let size = align_up(reserve.max(minimum), ALLOCATION_GRANULARITY)
            .ok_or_else(|| MwemuError::new("invalid PE32 stack reserve"))?;
        let base = if self.cfg.stack_addr == 0 {
            self.maps
                .find_free_range(
                    size,
                    ALLOCATION_GRANULARITY,
                    DEFAULT_STACK_BASE,
                    USER32_ADDRESS_LIMIT - PAGE_SIZE,
                )
                .ok_or_else(|| MwemuError::new("no free range for PE32 stack"))?
        } else {
            let base = self.cfg.stack_addr;
            if !base.is_multiple_of(PAGE_SIZE)
                || base
                    .checked_add(size)
                    .is_none_or(|end| end >= USER32_ADDRESS_LIMIT)
            {
                return Err(MwemuError::new("invalid configured PE32 stack address"));
            }
            base
        };
        let top = base + size;
        let limit = top - commit;
        self.maps
            .reserve_stack32(base, size, commit)
            .map_err(|err| MwemuError::new(&format!("cannot reserve PE32 stack: {err:?}")))?;
        self.cfg.stack_addr = base;
        self.regs_mut().set_esp(top - STARTUP_FRAME);
        self.regs_mut().set_ebp(top - size_of::<u32>() as u64);
        self.set_stack32_bounds(top, limit, base);
        Ok(())
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
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let level = log::max_level();
            let config = fast_log::Config::new()
                .format(CustomLogFormat::new())
                .console()
                .chan_len(Some(100000));
            if let Err(error) = fast_log::init(config) {
                // fast_log sets the global filter before it tries to register.
                log::set_max_level(level);
                log::debug!("keeping the existing logger: {error}");
            }
        });
    }

    /// Initialize windows simulator, this does like init_cpu() but also setup the windows memory.
    /// This require having the map files in place, otherwise use just init_cpu() but emu32() and
    /// emu64() already call init_cpu()
    /// This is called from load_code if the code is a PE or shellcode.
    /// load_code_bytes() and other loading ways don't call this, if you need windows simulation call this.
    pub fn init_win32(
        &mut self,
        clear_registers: bool,
        clear_flags: bool,
    ) -> Result<(), MwemuError> {
        self.init_windows_environment(clear_registers, clear_flags)?;
        if !self.cfg.arch.is_64bits() {
            self.init_win32_process_maps();
            self.init_stack32();
        }
        Ok(())
    }

    pub(super) fn init_windows_environment(
        &mut self,
        clear_registers: bool,
        clear_flags: bool,
    ) -> Result<(), MwemuError> {
        self.pos = 0;

        if !std::io::stdout().is_terminal() {
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
            self.instruction_state = InstructionState::default();
            if matches!(
                self.threads[self.current_thread_id].arch,
                ArchThreadState::X86 { .. }
            ) {
                let id = self.threads[self.current_thread_id].id;
                self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
            }
        }

        //log::trace!("initializing regs");
        if self.cfg.is_aarch64() {
            // AArch64: zero all registers; no x86 flags to clear.
            if clear_registers {
                *self.regs_aarch64_mut() = RegsAarch64::new();
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
                self.init_win_aarch64()?;
            } else {
                self.init_win32_mem64()?;
                self.init_stack64();
            }
        } else {
            // 32bits
            self.maps.is_64bits = false;
            self.regs_mut().sanitize32();
            self.init_win32_fixed_maps();
        }

        // Load banzai metadata only when 32-bit skip-unimplemented mode needs it.
        if !self.cfg.arch.is_64bits() && self.cfg.skip_unimplemented {
            self.banzai.load(&self.cfg.get_maps_folder("banzai.csv"));
        }

        Ok(())
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
        for address in self.heap_arenas[arena_idx].external_blocks() {
            let _ = self.maps.release_memory(address);
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
                ArchThreadState::X86 { .. }
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

    /// Initialize linux aarch64 simulation for ELF loading.
    pub fn init_linux64_aarch64(&mut self) {
        // Ensure thread context is aarch64
        if matches!(
            self.threads[self.current_thread_id].arch,
            ArchThreadState::X86 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        self.ensure_arch_state_aarch64();
        self.init_stack_aarch64();
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

    /// Populate argc, argv, envp and the auxiliary vector in the guest's word size.
    pub fn write_linux_stack_layout(
        &mut self,
        entry: u64,
        phdr_addr: u64,
        phentsize: u16,
        phnum: u16,
    ) {
        let _ = self.write_linux_exec_stack(entry, phdr_addr, phentsize, phnum, 0, None);
    }

    pub(crate) fn write_linux_exec_stack(
        &mut self,
        entry: u64,
        phdr_addr: u64,
        phentsize: u16,
        phnum: u16,
        interpreter_base: u64,
        exec_path: Option<&str>,
    ) -> Option<()> {
        let word_size = if self.cfg.arch == Arch::X86 { 4 } else { 8 };
        let program = if self.cfg.exe_name.is_empty() && exec_path.is_none() {
            "prog"
        } else {
            &self.cfg.exe_name
        };
        let mut args = vec![program.to_owned()];
        if let Some(files) = &self.linux_files {
            args.extend_from_slice(&files.args);
        } else {
            args.extend(self.cfg.arguments.split_whitespace().map(str::to_owned));
        }
        let mut cursor = self.sp();
        let env = self
            .linux_files
            .as_ref()
            .map(|fs| fs.env.as_slice())
            .unwrap_or_default();
        let mut push_string = |value: &str| -> Option<u64> {
            cursor = cursor.checked_sub(value.len() as u64 + 1)?;
            let written = self.maps.write_bytes(cursor, value.as_bytes())
                && self.maps.write_byte(cursor + value.len() as u64, 0);
            written.then_some(cursor)
        };
        let exec_name = match exec_path {
            Some(path) => Some(push_string(path)?),
            None => None,
        };
        let mut envp: Vec<_> = env
            .iter()
            .rev()
            .map(|s| push_string(s))
            .collect::<Option<_>>()?;
        let mut argv: Vec<_> = args
            .iter()
            .rev()
            .map(|s| push_string(s))
            .collect::<Option<_>>()?;
        envp.reverse();
        argv.reverse();
        let random = cursor.checked_sub(16)?;
        for i in 0..2 {
            let value = self.random_u64();
            self.maps.write_qword(random + 8 * i, value);
        }
        let name = exec_name.unwrap_or(argv[0]);
        let mut words = vec![argv.len() as u64];
        words.extend(argv);
        words.push(0);
        words.extend(envp);
        words.push(0);
        let auxv_index = words.len();
        words.extend([
            AT_PAGESZ,
            4096,
            AT_PHDR,
            phdr_addr,
            AT_PHENT,
            phentsize as u64,
            AT_PHNUM,
            phnum as u64,
            AT_BASE,
            interpreter_base,
            AT_ENTRY,
            entry,
            AT_RANDOM,
            random,
            AT_EXECFN,
            name,
        ]);
        if exec_path.is_some() {
            let id = GUEST_ID as u64;
            words.extend([
                AT_UID, id, AT_EUID, id, AT_GID, id, AT_EGID, id, AT_SECURE, 0, AT_CLKTCK, 100,
            ]);
        }
        words.extend([AT_NULL, 0]);
        let frame = random.checked_sub(word_size * words.len() as u64)?;
        let sp = frame & !15;
        if !self.maps.validate_write_range(sp, cursor - sp) {
            return None;
        }
        let auxv_address = sp + word_size * auxv_index as u64;
        let auxv_bytes = word_size as usize * (words.len() - auxv_index);
        self.set_sp(sp);
        for (index, word) in words.into_iter().enumerate() {
            let dest = sp + word_size * index as u64;
            if self.cfg.arch == Arch::X86 {
                self.maps.write_dword(dest, word as u32);
            } else {
                self.maps.write_qword(dest, word);
            }
        }
        self.save_linux_auxv(auxv_address, auxv_bytes);
        Some(())
    }

    /// Set up macOS AArch64 calling convention for _main(argc, argv, envp, apple).
    /// dyld calls _main with arguments in registers, not on the stack.
    pub fn write_macos_stack_layout(&mut self) {
        let sp = self.regs_aarch64().sp;

        // Build the argument list: argv[0] = filename, rest from cfg.arguments
        let mut args: Vec<String> = vec![self.cfg.filename.clone()];
        if !self.cfg.arguments.is_empty() {
            for a in self.cfg.arguments.split_whitespace() {
                args.push(a.to_string());
            }
        }
        let argc = args.len() as u64;

        // Write argument strings into the stack area
        let strings_base = sp + 0x200;
        let mut string_addrs: Vec<u64> = Vec::new();
        let mut string_off: u64 = 0;
        for arg in &args {
            let addr = strings_base + string_off;
            let bytes = arg.as_bytes();
            for (i, &b) in bytes.iter().enumerate() {
                self.maps.write_byte(addr + i as u64, b);
            }
            self.maps.write_byte(addr + bytes.len() as u64, 0);
            string_addrs.push(addr);
            string_off += (bytes.len() as u64) + 1;
        }

        // Write argv pointer array on the stack
        let argv_base = sp + 0x100;
        for (i, &addr) in string_addrs.iter().enumerate() {
            self.maps.write_qword(argv_base + (i as u64) * 8, addr);
        }
        // argv terminator
        self.maps
            .write_qword(argv_base + (string_addrs.len() as u64) * 8, 0);

        // envp = empty array (just NULL terminator)
        let envp_base = argv_base + ((string_addrs.len() as u64) + 1) * 8;
        self.maps.write_qword(envp_base, 0);

        // apple[] = empty array (just NULL terminator)
        let apple_base = envp_base + 8;
        self.maps.write_qword(apple_base, 0);

        // Set registers: _main(argc, argv, envp, apple)
        let regs = self.regs_aarch64_mut();
        regs.x[0] = argc;
        regs.x[1] = argv_base;
        regs.x[2] = envp_base;
        regs.x[3] = apple_base;
    }

    /// Initialize macOS aarch64 simulation for Mach-O loading.
    pub fn init_macos_aarch64(&mut self) {
        self.os = OperatingSystem::MacOS;

        // Ensure thread context is aarch64
        if matches!(
            self.threads[self.current_thread_id].arch,
            ArchThreadState::X86 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        self.init_stack_aarch64();
    }

    /// Initialize macOS x86_64 simulation for Mach-O loading.
    pub fn init_macos64(&mut self) {
        self.os = OperatingSystem::MacOS;
        self.maps.is_64bits = true;

        // Mach-O front-door loading can switch the emulator from an earlier
        // AArch64 session, so restore the x86 thread/register state explicitly.
        if matches!(
            self.threads[self.current_thread_id].arch,
            ArchThreadState::AArch64 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        self.instruction_state = InstructionState::default();
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
            if let Err(e) = self.maps.create_map(
                "linux_dynamic_stack",
                0x7ffffffde000,
                0x100000,
                Permission::READ_WRITE,
            ) {
                log::error!("cannot create linux_dynamic_stack map: {}", e);
                return;
            }
            if let Err(e) =
                self.maps
                    .create_map("dso_dyn", 0x7ffff7ffd000, 0x1000, Permission::READ_WRITE)
            {
                log::error!("cannot create dso_dyn map: {}", e);
            }
            if let Err(e) = self.maps.create_map(
                "linker",
                0x7ffff7ffd000 - 0x1000 - 0x10000,
                0x10000,
                Permission::READ_WRITE,
            ) {
                log::error!("cannot create linker map: {}", e);
            }
        } else {
            self.regs_mut().rsp = 0x7fffffffe270;
            if let Err(e) = self.maps.create_map(
                "linux_static_stack",
                0x7ffffffde000,
                0x100000,
                Permission::READ_WRITE,
            ) {
                log::error!("cannot create linux_static_stack map: {}", e);
                return;
            }
            if let Err(e) =
                self.maps
                    .create_map("dso", 0x7ffff7ffd000, 0x100000, Permission::READ_WRITE)
            {
                log::error!("cannot create dso map: {}", e);
            }
        }
        match self
            .maps
            .create_map("tls", 0x7ffff8fff000, 0xfff, Permission::READ_WRITE)
        {
            Ok(tls) => {
                let tls_path = self.cfg.get_maps_folder("tls.bin");
                tls.load(&tls_path);
            }
            Err(e) => {
                log::error!("cannot create tls map: {}", e);
            }
        }

        if dyn_link {
            //heap.set_base(0x555555579000);
        } else {
            let heap_sz = 0x885900 - 0x4b5000;
            self.heap_addr = match self.maps.alloc(heap_sz) {
                Some(addr) => addr,
                None => {
                    log::error!("cannot allocate heap for linux64");
                    return;
                }
            };
            match self
                .maps
                .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE)
            {
                Ok(heap) => {
                    heap.load("heap.bin");
                }
                Err(e) => {
                    log::error!("cannot create heap map: {}", e);
                    return;
                }
            }

            if self.heap_arenas.is_empty() {
                match O1Heap::new(self.heap_addr, heap_sz as u32) {
                    Ok(heap) => self.heap_arenas.push(Box::new(heap)),
                    Err(e) => log::error!("cannot init heap management: {}", e),
                }
            }
        }

        self.regs_mut().rbp = 0;

        // Linux x86-64 TCB (Thread Control Block)
        let tcb_addr: u64 = 0x7ffff9000000;
        let tcb_size: u64 = 0x1000;
        if let Err(e) =
            self.maps
                .create_map("linux_tcb", tcb_addr, tcb_size, Permission::READ_WRITE)
        {
            log::error!("cannot create linux_tcb map: {}", e);
            return;
        }
        self.maps.write_qword(tcb_addr, tcb_addr); // +0x00: self pointer
        self.maps.write_qword(tcb_addr + 0x10, tcb_addr); // +0x10: self pointer
        self.maps.write_qword(tcb_addr + 0x28, 0xdeadbeefcafebabe); // +0x28: stack canary
        self.regs_mut().fs = tcb_addr;
    }

    pub fn init_win32_mem32(&mut self) {
        self.init_win32_fixed_maps();
        self.init_win32_process_maps();
    }

    fn init_win32_fixed_maps(&mut self) {
        self.maps.is_64bits = self.cfg.arch.is_64bits();
        kuser_shared::init_kuser_shared_data(self);
    }

    pub(super) fn init_win32_process_maps(&mut self) {
        peb32::init_peb(self);
        winapi32::kernel32::load_library(self, "ntdll.dll");
        self.syscall_name_by_real = index_syscall_names(self);
        let ntdll_base = self
            .maps
            .get_map_by_name("ntdll.pe")
            .map(|m| m.get_base())
            .unwrap_or(0);
        if ntdll_base != 0 {
            peb32::update_peb_image_base(self, ntdll_base as u32);
        }

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

    /// Initialize Windows ARM64 session: sets up aarch64 stack and full PEB64/TEB64/LDR64
    /// memory layout. Arch state and thread context must already be switched to AArch64
    /// before calling this (done at the top of init_win32).
    ///
    /// Reuses the same 64-bit Windows memory setup as x86_64 since PEB64/TEB64/LDR
    /// structures are architecture-neutral; the difference is register semantics and stack init.
    fn init_win_aarch64(&mut self) -> Result<(), MwemuError> {
        // Set up the aarch64 stack first (uses SP register instead of RSP/RBP)
        // so that the stack map exists when init_win32_mem64 writes TEB stack bounds.
        self.init_stack_aarch64();

        // Set up the 64-bit Windows memory (PEB64, TEB64, LDR, DLLs, heap)
        self.init_win32_mem64()
    }

    /// This is called from init(), this setup the 64bits windows memory simulation.
    pub fn init_win32_mem64(&mut self) -> Result<(), MwemuError> {
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
            syscall64::build_syscall_translation_table(self);

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
                    mem.add_permission(Permission::WRITE);
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

            return Ok(());
        }

        // Update exe_name from the real EXE basename before PEB init writes it.
        peb64::refresh_exe_name_from_filename(self);
        peb64::init_peb(self);
        kuser_shared::init_kuser_shared_data(self);

        let mut metadata = Vec::new();
        let core_libraries = ["ntdll.dll", "kernel32.dll", "kernelbase.dll"];

        // Map all system libraries before binding imports between them.
        for dll in core_libraries {
            self.ensure_maps_dll(dll); // fetch from the symbol server if missing
            let filepath = self.cfg.get_maps_folder(dll);
            log::debug!("mapping base lib64: {}", filepath);
            metadata.push((dll.to_string(), self.map_dll_pe64(&filepath)?));
        }

        let mut dependencies: BTreeSet<String> = BTreeSet::new();
        for (_, dll) in &metadata {
            for mut dep in dll.image.get_dependencies() {
                dep = dep.to_lowercase();
                if !dep.ends_with(".dll") {
                    dep.push_str(".dll");
                }
                if !core_libraries.contains(&dep.as_str()) {
                    dependencies.insert(dep);
                }
            }
        }

        for dll in dependencies {
            self.ensure_maps_dll(&dll); // fetch from the symbol server if missing
            let filepath = self.cfg.get_maps_folder(&dll);
            log::debug!("mapping dependency {}", filepath);
            metadata.push((dll.to_string(), self.map_dll_pe64(&filepath)?));
        }

        for (name, dll) in &metadata {
            log::debug!("dynamic linking {}", name);
            peb64::dynamic_link_module(dll.base, dll.image.get_pe_off(), name, self);
        }

        // Relocations and exports are ready for every dependency.
        for (name, dll) in &mut metadata {
            log::debug!("iat binding {}", name);
            dll.image.iat_binding(&dll.raw, self, dll.base);
            dll.image.delay_load_binding(&dll.raw, self, dll.base);
        }
        log::debug!("win32 64bits base libs ok.");

        syscall64::build_syscall_translation_table(self);
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
        self.maps
            .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE)
            .map_err(|error| MwemuError::new(&format!("cannot create heap map: {error}")))?;

        if self.heap_arenas.is_empty() {
            let heap = O1Heap::new(self.heap_addr, heap_sz as u32).map_err(|error| {
                MwemuError::new(&format!("cannot initialize process heap: {error}"))
            })?;
            self.heap_arenas.push(Box::new(heap));

            // Reserve the first 0x800 bytes for the _HEAP header so the
            // allocator never hands them out and overwrites the struct fields.
            self.heap_arenas[0].allocate(0x800);

            let heap_handle = HeapHandle::new(0x0, heap_sz as usize, 0, self.heap_arenas.len() - 1);
            let key = self.handle_management.insert_heap_handle(heap_handle);
            self.handle_management
                .set_heap_base_addr(key, self.heap_addr);
            let process_key = self.handle_management.get_or_insert_process_heap();
            debug_assert_eq!(key, process_key);
        }

        // Populate the _HEAP structure via Heap64 (see docs/ANTI_EMU_HEAP.md).
        let heap_struct = structures::Heap64::new(self.heap_addr, heap_sz);
        let heap_map = self.maps.get_mem_mut(".heap");
        heap_struct.save(heap_map);
        Ok(())
    }
}
