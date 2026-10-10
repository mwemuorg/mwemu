//! Loading a macOS kernel extension: the `MH_KEXT_BUNDLE` path.
//!
//! A kext is the Mach-O counterpart of a Linux `.ko` — an object the kernel
//! links into its own address space and then calls back into. Unlike a `.ko`
//! (an ET_REL with section-level relocations) a modern arm64e kext is already
//! a laid-out image: its segments carry final, image-relative vmaddrs and all
//! its fixups are expressed as a chained-pointer list (`LC_DYLD_CHAINED_FIXUPS`,
//! pointer format `ARM64E_KERNEL`). So loading one is:
//!
//! 1. place every segment contiguously at the kernel module base,
//! 2. apply the chained fixups — rebases become `base + target`, binds become
//!    the address of an interceptable kernel stub,
//! 3. read `kmod_info` (located via the symbol table) for the module name and
//!    its `start`/`stop` entry points, the kext equivalent of
//!    `init_module`/`cleanup_module`.
//!
//! Nothing runs as a side effect of loading. `start` is invoked explicitly by
//! [`Emu::run_module_init`], exactly as on the Linux side.

use rs_header::elf::relocatable::RelSymbol;

use crate::arch::Arch;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::kernel::KernelOs;
use crate::loaders::macho::macho64::{Macho64, prot_to_permission};

/// `kmod_info_t` field offsets on 64-bit (the struct is `#pragma pack(4)`):
/// `char name[64]` at 16, and the `start`/`stop` function pointers at the end.
const KMOD_NAME_OFFSET: u64 = 16;
const KMOD_START_OFFSET: u64 = 180;
const KMOD_STOP_OFFSET: u64 = 188;

impl Emu {
    /// Load a macOS kext and link it against the emulated XNU kernel.
    ///
    /// Returns the module's base address. `start` is *not* executed; call
    /// [`Emu::run_module_init`] for that.
    pub fn load_kext_macho64(&mut self, filename: &str) -> Result<u64, MwemuError> {
        let macho = Macho64::parse(filename)?;

        // arm64e kexts carry chained fixups; x86_64 kexts use classic external
        // relocations (LC_DYSYMTAB), a separate path not built yet.
        if !self.cfg.arch.is_aarch64() {
            self.cfg.arch = Arch::Aarch64;
        }
        self.ensure_arch_state_aarch64();
        self.maps.is_64bits = true;
        self.filename = filename.to_string();
        self.cfg.filename = filename.to_string();

        self.kernel_init(KernelOs::MacOS);
        let base = self
            .kernel
            .as_ref()
            .expect("kernel env present")
            .layout
            .module_base;

        // Image is contiguous from its lowest segment vmaddr; a kext's is 0.
        let vmaddr_base = macho
            .segments
            .iter()
            .filter(|s| s.vmsize > 0)
            .map(|s| s.vmaddr)
            .min()
            .unwrap_or(0);

        // --- place the segments ----------------------------------------------
        let mut image_end = base;
        let mut text_ranges: Vec<(u64, u64)> = Vec::new();
        for seg in &macho.segments {
            if seg.vmsize == 0 || seg.name == "__LINKEDIT" {
                continue;
            }
            let seg_addr = base + (seg.vmaddr - vmaddr_base);
            let perm = prot_to_permission(seg.initprot);
            if seg.initprot & 0x4 != 0 {
                text_ranges.push((seg_addr, seg_addr + seg.vmsize));
            }
            let mem = self
                .maps
                .create_map(&format!("kext{}", seg.name), seg_addr, seg.vmsize, perm)
                .map_err(|e| MwemuError::new(&format!("cannot map kext segment: {}", e)))?;
            if !seg.data.is_empty() {
                mem.force_write_bytes(seg_addr, &seg.data);
            }
            image_end = image_end.max(seg_addr + seg.vmsize);
        }

        // --- apply the chained fixups ----------------------------------------
        let (imports, binds, rebases) = macho.parse_chained_fixups();
        for bind in &binds {
            let name = imports
                .get(bind.import_ordinal as usize)
                .map(|i| i.name.as_str())
                .unwrap_or("");
            // Kernel surface symbols are spelled without the C leading '_'.
            let sym = name.strip_prefix('_').unwrap_or(name);
            match self.kernel_resolve_import(sym) {
                Some(stub) => {
                    self.maps.write_qword(base + bind.got_vmaddr, stub);
                }
                None => log::warn!("kext import {} did not resolve", name),
            }
        }
        for rebase in &rebases {
            // Every kext rebase is image-relative.
            let value = (base + rebase.target) | (rebase.high8 << 56);
            self.maps.write_qword(base + rebase.vmaddr, value);
        }

        // --- locate kmod_info and the entry points ---------------------------
        let exports = macho.get_exports();
        let symbols: Vec<RelSymbol> = exports
            .iter()
            .map(|(name, off)| {
                let addr = base + off;
                RelSymbol {
                    name: name.clone(),
                    addr,
                    size: 0,
                    is_func: text_ranges.iter().any(|(s, e)| addr >= *s && addr < *e),
                    is_global: true,
                }
            })
            .collect();

        let kmod_off = exports
            .iter()
            .find(|(name, _)| name == "_kmod_info")
            .map(|(_, off)| *off)
            .ok_or_else(|| MwemuError::new("kext has no _kmod_info symbol"))?;
        let kmod_addr = base + kmod_off;

        // start/stop were just rebased to absolute addresses in the fixup pass.
        let start = self.maps.read_qword(kmod_addr + KMOD_START_OFFSET);
        let stop = self.maps.read_qword(kmod_addr + KMOD_STOP_OFFSET);
        let name = self.maps.read_string(kmod_addr + KMOD_NAME_OFFSET);
        let name = if name.is_empty() {
            std::path::Path::new(filename)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "kext".to_string())
        } else {
            name
        };

        log::info!(
            "loaded kext '{}' at 0x{:x} ({} bytes, start=0x{:x} stop=0x{:x})",
            name,
            base,
            image_end - base,
            start.unwrap_or(0),
            stop.unwrap_or(0)
        );

        let module = crate::kernel::ModuleImage {
            name,
            base,
            size: image_end - base,
            init: start.filter(|&a| a != 0),
            exit: stop.filter(|&a| a != 0),
            sections: Vec::new(),
            symbols,
            unresolved: Vec::new(),
        };
        self.kernel.as_mut().expect("kernel env present").module = module;

        self.base = base;
        self.macho64 = Some(macho);
        Ok(base)
    }
}
