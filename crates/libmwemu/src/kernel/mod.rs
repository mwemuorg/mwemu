//! Kernel-mode emulation: running pieces of a driver instead of a program.
//!
//! A driver is not a process. There is no libc, no PEB, no dynamic loader and
//! no entry point that "runs" — it is a relocatable object the OS links into
//! its own address space and then calls back into. So this subsystem provides
//! the three things a driver actually needs and a user-mode target does not:
//!
//! 1. **A linker.** `.ko` / `.sys` images are ET_REL / COFF objects; their
//!    sections must be placed and their relocations applied before anything can
//!    execute. ELF parsing stays in `rs-header`; the placement policy is in
//!    [`layout`].
//! 2. **A kernel to call.** Every imported symbol gets a stub address in a
//!    synthetic "kernel text" region. A call landing there is intercepted and
//!    routed to a Rust implementation, the same trick the winapi layer uses.
//! 3. **An allocator with a memory.** Driver bugs are lifetime bugs, so the
//!    slab is modelled explicitly: chunks are tracked, freed chunks go to
//!    quarantine instead of being recycled, and [`guard`] reports what touches
//!    them. That is what makes a use-after-free observable rather than lucky.
//!
//! The per-OS API surfaces live in [`linux`], [`windows`] and [`macos`]; only
//! the symbol tables and the handlers differ, everything above is shared.

pub mod guard;
pub mod heap;
pub mod layout;
pub mod linux;
pub mod macos;
pub mod windows;

use std::collections::HashMap;

use rs_header::elf::relocatable::{RelSection, RelSymbol};

use crate::emu::Emu;
use crate::kernel::guard::Finding;
use crate::kernel::heap::{KernelHeap, Region};
use crate::kernel::layout::{KernelLayout, STUB_SLOT};
use crate::maps::mem64::Permission;

/// Which kernel's API surface a loaded driver targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelOs {
    Linux,
    Windows,
    MacOS,
}

impl KernelOs {
    pub fn label(self) -> &'static str {
        match self {
            KernelOs::Linux => "linux",
            KernelOs::Windows => "windows",
            KernelOs::MacOS => "macos",
        }
    }
}

/// A loaded driver image.
#[derive(Debug, Default)]
pub struct ModuleImage {
    pub name: String,
    pub base: u64,
    pub size: u64,
    /// Module entry point (`init_module` / `DriverEntry` / kext start).
    pub init: Option<u64>,
    /// Module teardown (`cleanup_module` / unload routine).
    pub exit: Option<u64>,
    pub sections: Vec<RelSection>,
    pub symbols: Vec<RelSymbol>,
    /// Imports the kernel surface does not implement yet.
    pub unresolved: Vec<String>,
}

impl ModuleImage {
    /// Address of a symbol the module defines — how a caller reaches an ioctl
    /// handler, a file operation, or any other function to drive directly.
    pub fn symbol(&self, name: &str) -> Option<u64> {
        self.symbols
            .iter()
            .filter(|s| s.name == name)
            .max_by_key(|s| s.is_global)
            .map(|s| s.addr)
    }
}

/// A `kmem_cache` created by the driver.
#[derive(Debug, Clone)]
pub struct KmemCache {
    pub handle: u64,
    pub name: String,
    pub obj_size: u64,
}

/// A callback the driver postponed: a work item, a timer, an RCU callback.
///
/// Deferred frees are the shape of most kernel use-after-free bugs, so these
/// are queued rather than run inline — the gap between "queued" and "drained"
/// is the window the bug lives in.
#[derive(Debug, Clone)]
pub struct DeferredCall {
    /// What queued it: `work`, `delayed_work`, `timer`, `rcu`.
    pub kind: String,
    /// Callback address inside the module.
    pub func: u64,
    /// Single argument the callback receives.
    pub arg: u64,
}

/// Everything the emulated kernel owns for one run.
#[derive(Debug)]
pub struct KernelEnv {
    pub os: KernelOs,
    pub layout: KernelLayout,
    pub heap: KernelHeap,
    pub module: ModuleImage,
    pub findings: Vec<Finding>,
    /// Stub address -> imported symbol name.
    pub stub_by_addr: HashMap<u64, String>,
    /// Imported symbol name -> stub address.
    pub stub_by_name: HashMap<String, u64>,
    /// Imported *data* symbol name -> address in the kernel data region.
    pub data_by_name: HashMap<String, u64>,
    /// Lines the driver emitted through printk / DbgPrint / IOLog.
    pub log: Vec<String>,
    /// Caches created through `kmem_cache_create`, keyed by handle.
    pub caches: HashMap<u64, KmemCache>,
    /// Imported symbols that were called but have no implementation.
    pub unimplemented: Vec<String>,
    /// Callbacks queued by the driver and not run yet.
    pub deferred: Vec<DeferredCall>,
    next_stub: u64,
    next_data: u64,
}

impl KernelEnv {
    pub fn new(os: KernelOs) -> KernelEnv {
        let layout = KernelLayout::for_os(os);
        KernelEnv {
            os,
            layout,
            heap: KernelHeap::new(
                layout.heap_base,
                layout.heap_size,
                layout.vmalloc_base,
                layout.vmalloc_size,
            ),
            module: ModuleImage::default(),
            findings: Vec::new(),
            stub_by_addr: HashMap::new(),
            stub_by_name: HashMap::new(),
            data_by_name: HashMap::new(),
            log: Vec::new(),
            caches: HashMap::new(),
            unimplemented: Vec::new(),
            deferred: Vec::new(),
            next_stub: layout.stub_base,
            next_data: layout.data_base,
        }
    }

    /// Name of the kernel API a stub address stands for.
    pub fn symbol_at(&self, addr: u64) -> Option<&str> {
        self.stub_by_addr.get(&addr).map(|s| s.as_str())
    }
}

impl Emu {
    /// Bring up the emulated kernel address space: stub area, data area and a
    /// kernel stack. Idempotent per emulator instance.
    pub fn kernel_init(&mut self, os: KernelOs) {
        let layout = KernelLayout::for_os(os);

        // Stub area: every slot is a lone `ret`. Calls here are intercepted
        // before a byte executes, but a stub reached some other way (a stale
        // function pointer, say) must still unwind cleanly instead of running
        // into the next symbol's slot.
        if self.maps.get_map_by_name("kernel.stubs").is_none() {
            let stubs = self
                .maps
                .create_map(
                    "kernel.stubs",
                    layout.stub_base,
                    layout.stub_size,
                    Permission::READ_EXECUTE,
                )
                .expect("cannot create kernel.stubs map");
            let base = stubs.get_base();
            let filler = vec![0xc3u8; layout.stub_size as usize];
            stubs.force_write_bytes(base, &filler);
        }

        if self.maps.get_map_by_name("kernel.retpad").is_none() {
            let pad = self
                .maps
                .create_map(
                    "kernel.retpad",
                    layout.retpad(),
                    0x1000,
                    Permission::READ_EXECUTE,
                )
                .expect("cannot create kernel.retpad map");
            let base = pad.get_base();
            // `hlt`: returning here means the driver call is over and the CPU
            // has nothing left to do, which is exactly what halting says.
            pad.force_write_bytes(base, &[0xf4u8; 0x1000]);
        }

        if self.maps.get_map_by_name("kernel.data").is_none() {
            self.maps
                .create_map(
                    "kernel.data",
                    layout.data_base,
                    layout.data_size,
                    Permission::READ_WRITE,
                )
                .expect("cannot create kernel.data map");
        }

        if self.maps.get_map_by_name("kernel.stack").is_none() {
            self.maps
                .create_map(
                    "kernel.stack",
                    layout.stack_base,
                    layout.stack_size,
                    Permission::READ_WRITE,
                )
                .expect("cannot create kernel.stack map");
        }

        // Start mid-stack: kernel code both pushes and, through our API stubs,
        // occasionally reads a little above the current frame.
        let sp = layout.stack_base + layout.stack_size / 2;
        if self.cfg.arch.is_aarch64() {
            self.regs_aarch64_mut().sp = sp;
        } else {
            self.regs_mut().rsp = sp;
            self.regs_mut().rbp = sp;
        }

        self.kernel = Some(Box::new(KernelEnv::new(os)));
        self.kernel_guard = true;
        self.os = match os {
            KernelOs::Linux => crate::arch::OperatingSystem::Linux,
            KernelOs::Windows => crate::arch::OperatingSystem::Windows,
            KernelOs::MacOS => crate::arch::OperatingSystem::MacOS,
        };
    }

    /// Address standing for an imported kernel *function*, allocating a stub
    /// slot the first time the symbol is seen.
    pub fn kernel_stub_for(&mut self, name: &str) -> Option<u64> {
        let kernel = self.kernel.as_mut()?;
        if let Some(addr) = kernel.stub_by_name.get(name) {
            return Some(*addr);
        }
        let addr = kernel.next_stub;
        if addr + STUB_SLOT > kernel.layout.stub_base + kernel.layout.stub_size {
            log::error!("kernel stub area exhausted, cannot import {}", name);
            return None;
        }
        kernel.next_stub += STUB_SLOT;
        kernel.stub_by_name.insert(name.to_string(), addr);
        kernel.stub_by_addr.insert(addr, name.to_string());
        Some(addr)
    }

    /// Address standing for an imported kernel *variable* (`jiffies`,
    /// `kmalloc_caches`, …), backed by zeroed storage in the data region.
    pub fn kernel_data_for(&mut self, name: &str, size: u64) -> Option<u64> {
        let kernel = self.kernel.as_mut()?;
        if let Some(addr) = kernel.data_by_name.get(name) {
            return Some(*addr);
        }
        let addr = kernel.next_data.next_multiple_of(16);
        if addr + size > kernel.layout.data_base + kernel.layout.data_size {
            log::error!("kernel data area exhausted, cannot import {}", name);
            return None;
        }
        kernel.next_data = addr + size;
        kernel.data_by_name.insert(name.to_string(), addr);
        Some(addr)
    }

    /// Resolve one import of a driver image to an address in the emulated
    /// kernel: readable storage for a variable, an interceptable stub for a
    /// function. This is the emulator's `resolve_symbol()`.
    pub fn kernel_resolve_import(&mut self, name: &str) -> Option<u64> {
        let os = self.kernel.as_ref()?.os;
        let data_size = match os {
            KernelOs::Linux => linux::data_symbol_size(name),
            // The Windows and macOS surfaces import data through the image's
            // import table rather than by name, so nothing is classified here
            // yet; a function stub is the safe default.
            KernelOs::Windows | KernelOs::MacOS => None,
        };
        match data_size {
            Some(size) => self.kernel_data_for(name, size),
            None => self.kernel_stub_for(name),
        }
    }

    /// Queue a callback the driver postponed.
    pub fn kernel_defer(&mut self, call: DeferredCall) {
        if call.func == 0 {
            return;
        }
        log::info!(
            "kernel: queued {} callback 0x{:x}(0x{:x})",
            call.kind,
            call.func,
            call.arg
        );
        if let Some(kernel) = self.kernel.as_mut() {
            kernel.deferred.push(call);
        }
    }

    /// Run every queued callback, in the order they were queued.
    ///
    /// Callbacks may queue more work, so the queue is drained rather than
    /// iterated; the bound stops a driver that re-arms a timer forever.
    pub fn kernel_run_deferred(&mut self) -> usize {
        const MAX_ROUNDS: usize = 64;
        let mut ran = 0;
        for _ in 0..MAX_ROUNDS {
            let Some(call) = self.kernel.as_mut().and_then(|k| {
                if k.deferred.is_empty() {
                    None
                } else {
                    Some(k.deferred.remove(0))
                }
            }) else {
                break;
            };
            log::info!(
                "kernel: running {} callback 0x{:x}(0x{:x})",
                call.kind,
                call.func,
                call.arg
            );
            if let Err(e) = self.kernel_call(call.func, &[call.arg]) {
                log::warn!("kernel: {} callback failed: {}", call.kind, e);
            }
            ran += 1;
        }
        ran
    }

    /// Hand out a chunk from an emulated kernel allocator: reserve the address,
    /// map it, optionally zero it, and record its provenance.
    pub fn kernel_alloc(
        &mut self,
        region: Region,
        req_size: u64,
        cache: &str,
        api: &str,
        zeroed: bool,
    ) -> u64 {
        let pos = self.pos;
        let rip = self.pc();
        let Some(kernel) = self.kernel.as_mut() else {
            return 0;
        };
        let Some(chunk) = kernel
            .heap
            .record_alloc(region, req_size, cache, api, pos, rip)
        else {
            log::warn!("{}: {} region exhausted", api, region.label());
            return 0;
        };
        let (addr, size, map_name) = (chunk.addr, chunk.size, chunk.map_name.clone());

        let mem = self
            .maps
            .create_map(&map_name, addr, size, Permission::READ_WRITE)
            .expect("cannot map kernel allocation");
        if zeroed {
            mem.write_bytes(addr, &vec![0u8; size as usize]);
        } else {
            // Uninitialised slab memory is not zero on a real kernel; a
            // recognisable pattern (SLUB's POISON_INUSE) makes a
            // use-before-init obvious in a dump.
            mem.write_bytes(addr, &vec![0x5au8; size as usize]);
        }
        addr
    }

    /// Return a chunk to the emulated allocator: quarantine it, poison it, and
    /// report a double or invalid free.
    ///
    /// The map is deliberately kept. A real slab would recycle the memory
    /// immediately, which is exactly what hides use-after-free; keeping the
    /// chunk mapped and poisoned lets execution continue so the whole misuse
    /// chain shows up in one run.
    pub fn kernel_free(&mut self, ptr: u64, api: &str) -> bool {
        if ptr == 0 {
            return true; // kfree(NULL) is legal, like free(NULL)
        }
        let pos = self.pos;
        let rip = self.pc();

        let Some(kernel) = self.kernel.as_ref() else {
            return false;
        };

        let Some(idx) = kernel.heap.index_of_base(ptr) else {
            // Not the base of any chunk: either an interior pointer (which the
            // real allocator would also reject) or something that never came
            // from us.
            let origin = kernel
                .heap
                .chunk_at(ptr)
                .map(guard::ChunkOrigin::from)
                .unwrap_or_default();
            self.kernel_report(guard::FindingKind::InvalidFree, rip, ptr, 0, origin);
            return false;
        };

        let chunk = kernel.heap.get(idx);
        if chunk.is_freed() {
            let origin = guard::ChunkOrigin::from(chunk);
            self.kernel_report(guard::FindingKind::DoubleFree, rip, ptr, 0, origin);
            return false;
        }
        let (addr, size) = (chunk.addr, chunk.size);

        self.kernel
            .as_mut()
            .expect("kernel env present")
            .heap
            .record_free(idx, api, pos, rip);

        // SLUB's POISON_FREE: freed memory reads back as 0x6b6b6b6b, the value
        // that shows up in a real kernel oops when a stale pointer is used.
        if let Some(mem) = self.maps.get_mem_by_addr_mut(addr) {
            mem.force_write_bytes(addr, &vec![0x6bu8; size as usize]);
        }
        true
    }

    /// Handle a control transfer while a driver is loaded.
    ///
    /// Returns true when execution has been redirected (the caller must stop
    /// decoding at the old address). Anything inside the module image is left
    /// alone; only the synthetic kernel text is intercepted.
    pub fn kernel_set_pc(&mut self, addr: u64) -> bool {
        let Some(kernel) = self.kernel.as_ref() else {
            return false;
        };
        if !kernel.layout.is_stub(addr) {
            if !self.maps.is_mapped(addr) {
                // Control flow left the kernel entirely. When every byte of the
                // target is the slab free poison, the pointer was read out of a
                // quarantined object — the payoff of a use-after-free, and worth
                // reporting as one rather than as a generic bad branch.
                if guard::is_free_poison(addr) {
                    let rip = self.pc();
                    self.kernel_report(
                        guard::FindingKind::FreedFunctionPointerCall,
                        rip,
                        addr,
                        0,
                        guard::ChunkOrigin::default(),
                    );
                } else {
                    log::error!(
                        "kernel: branch to unmapped address 0x{:x} from 0x{:x}",
                        addr,
                        self.pc()
                    );
                }
                self.stop();
                self.force_break = true;
                return true;
            }
            // Inside the driver's own image (or its data): ordinary flow.
            if self.cfg.arch.is_aarch64() {
                self.regs_aarch64_mut().pc = addr;
            } else {
                self.regs_mut().rip = addr;
            }
            return true;
        }

        let symbol = kernel.symbol_at(addr).unwrap_or("<unknown>").to_string();

        // Retpoline thunks are a jump, not a call: they transfer to the
        // register instead of returning, so they must be handled before the
        // return address is consumed.
        if let Some(reg) = symbol.strip_prefix("__x86_indirect_thunk_") {
            return self.kernel_indirect_thunk(reg);
        }

        if self.cfg.verbose >= 1 {
            log::info!(
                "{}** {} kernel API: {} {}",
                self.colors.light_red,
                self.pos,
                symbol,
                self.colors.nc
            );
        }

        // Consume the return address the same way the user-mode gateways do,
        // so the handler only has to produce a return value.
        if self.cfg.arch.is_aarch64() {
            let lr = self.regs_aarch64().x[30];
            self.gateway_return = lr;
            self.regs_aarch64_mut().pc = lr;
        } else {
            self.gateway_return = self.stack_pop64(false).unwrap_or(0);
            self.regs_mut().rip = self.gateway_return;
        }

        let os = self.kernel.as_ref().expect("kernel env present").os;
        let handled = match os {
            KernelOs::Linux => linux::gateway(&symbol, self),
            KernelOs::Windows => windows::gateway(&symbol, self),
            KernelOs::MacOS => macos::gateway(&symbol, self),
        };

        if !handled {
            log::warn!(
                "kernel API {} is not implemented — returning 0 and continuing",
                symbol
            );
            let kernel = self.kernel.as_mut().expect("kernel env present");
            if !kernel.unimplemented.iter().any(|s| s == &symbol) {
                kernel.unimplemented.push(symbol);
            }
            self.set_kernel_ret(0);
        }

        self.force_break = true;
        true
    }

    /// `__x86_indirect_thunk_<reg>`: a retpoline that jumps to the register.
    fn kernel_indirect_thunk(&mut self, reg: &str) -> bool {
        let target = match reg {
            "rax" => self.regs().rax,
            "rbx" => self.regs().rbx,
            "rcx" => self.regs().rcx,
            "rdx" => self.regs().rdx,
            "rsi" => self.regs().rsi,
            "rdi" => self.regs().rdi,
            "rbp" => self.regs().rbp,
            "r8" => self.regs().r8,
            "r9" => self.regs().r9,
            "r10" => self.regs().r10,
            "r11" => self.regs().r11,
            "r12" => self.regs().r12,
            "r13" => self.regs().r13,
            "r14" => self.regs().r14,
            "r15" => self.regs().r15,
            other => {
                log::warn!("unsupported retpoline thunk register {}", other);
                return false;
            }
        };
        // A call through a freed object's function pointer is the payoff of a
        // use-after-free; attribute it before transferring control.
        self.kernel_check_call_target(target);
        self.regs_mut().rip = target;
        self.force_break = true;
        true
    }

    /// Flag an indirect branch whose target was read out of quarantined memory.
    pub fn kernel_check_call_target(&mut self, target: u64) {
        let rip = self.pc();
        let origin = match self.kernel.as_ref() {
            Some(k) => match k.heap.chunk_at(target) {
                Some(c) if c.is_freed() => guard::ChunkOrigin::from(c),
                _ => return,
            },
            None => return,
        };
        self.kernel_report(
            guard::FindingKind::FreedFunctionPointerCall,
            rip,
            target,
            0,
            origin,
        );
    }

    /// Call into driver code with the kernel's calling convention.
    ///
    /// Linux and XNU kernel code is plain SysV / AAPCS64; a Windows driver uses
    /// the Microsoft x64 convention, the same one user-mode PE code uses.
    pub fn kernel_call(&mut self, addr: u64, args: &[u64]) -> Result<u64, crate::err::MwemuError> {
        // Return to the pad so the callee's `ret` lands on a mapped address
        // that ends the run cleanly.
        if let Some(kernel) = self.kernel.as_ref() {
            let retpad = kernel.layout.retpad();
            if self.cfg.arch.is_aarch64() {
                self.regs_aarch64_mut().pc = retpad;
            } else {
                self.regs_mut().rip = retpad;
            }
        }

        if self.cfg.arch.is_aarch64() {
            self.aarch64_call64(addr, args)
        } else if matches!(self.kernel.as_ref().map(|k| k.os), Some(KernelOs::Windows)) {
            self.call64(addr, args)
        } else {
            self.linux_call64(addr, args)
        }
    }

    /// Set the return value of a kernel API, whichever architecture is active.
    pub fn set_kernel_ret(&mut self, value: u64) {
        if self.cfg.arch.is_aarch64() {
            self.regs_aarch64_mut().x[0] = value;
        } else {
            self.regs_mut().rax = value;
        }
    }

    /// Read argument `idx` of a kernel API call (SysV / AAPCS64 order).
    pub fn kernel_arg(&self, idx: usize) -> u64 {
        crate::api::abi::ApiAbi::from_emu(self).arg(self, idx)
    }

    /// Append a line to the emulated kernel log (`dmesg`).
    pub fn kernel_log_line(&mut self, line: String) {
        log::info!(
            "{}[kernel] {}{}",
            self.colors.light_cyan,
            line,
            self.colors.nc
        );
        if let Some(kernel) = self.kernel.as_mut() {
            kernel.log.push(line);
        }
    }
}
