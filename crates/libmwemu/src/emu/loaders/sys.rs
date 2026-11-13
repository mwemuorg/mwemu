//! Loading a Windows driver: the `.sys` path.
//!
//! A `.sys` is an ordinary PE64 image with `DriverEntry` as its entry point
//! and imports from `ntoskrnl.exe`, `HAL.dll`, `FLTMGR.SYS`, `WDFLDR.SYS` and
//! friends. Unlike a `.ko` it is already laid out, so loading is the PE path
//! with kernel-space placement:
//!
//! 1. map the sections at the kernel module base and apply base relocations,
//! 2. bind every import to the emulated kernel — an interceptable stub for a
//!    function, zeroed storage for a data export like `IoDeviceObjectType`,
//! 3. record `DriverEntry` as the module's init.
//!
//! Nothing executes as a side effect of loading. `DriverEntry` is run
//! explicitly by [`Emu::run_module_init`] / [`Emu::run_driver_entry`], which
//! also builds the `DRIVER_OBJECT` the driver expects.

use rs_header::elf::relocatable::{RelSection, RelSymbol};
use rs_header::pe::export_index::build_image_export_index;
use rs_header::pe::pe64::PE64;
use rs_header::pe::readers::{read_c_string, read_u64_le};
use rs_header::pe::shared::IMAGE_DIRECTORY_ENTRY_EXPORT;

use crate::arch::Arch;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::kernel::{KernelOs, windows};

const IMAGE_SUBSYSTEM_NATIVE: u16 = 1;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
const IMAGE_SCN_MEM_EXECUTE: u32 = 0x2000_0000;
const IMAGE_SCN_MEM_READ: u32 = 0x4000_0000;
const IMAGE_SCN_MEM_WRITE: u32 = 0x8000_0000;

/// Libraries only a kernel-mode image imports.
const KERNEL_LIBRARIES: &[&str] = &[
    "ntoskrnl.exe",
    "hal.dll",
    "fltmgr.sys",
    "wdfldr.sys",
    "ndis.sys",
    "ksecdd.sys",
    "cng.sys",
    "netio.sys",
    "storport.sys",
    "wdf01000.sys",
    "tdi.sys",
    "fwpkclnt.sys",
    "ntstrsafe.sys",
    "clfs.sys",
    "ci.dll",
    "bootvid.dll",
    "kdcom.dll",
    "pshed.dll",
    "hidclass.sys",
    "usbd.sys",
    "scsiport.sys",
    "videoprt.sys",
    "dxgkrnl.sys",
    "ks.sys",
    "portcls.sys",
];

/// Is this PE64 a kernel-mode driver rather than a user-mode program?
///
/// The subsystem says *native*, and the imports come from the kernel: a
/// native user-mode image (smss, autochk) imports `ntdll.dll` instead.
pub fn is_kernel_driver(raw: &[u8]) -> bool {
    if !PE64::is_pe64(raw) {
        return false;
    }
    let pe = PE64::parse("", raw);
    if pe.fh.machine != IMAGE_FILE_MACHINE_AMD64 || pe.opt.subsystem != IMAGE_SUBSYSTEM_NATIVE {
        return false;
    }
    let imports: Vec<String> = pe
        .image_import_descriptor
        .iter()
        .map(|d| d.name.trim().to_ascii_lowercase())
        .collect();
    if imports.iter().any(|n| n == "ntdll.dll") {
        return false;
    }
    imports.is_empty()
        || imports
            .iter()
            .any(|n| KERNEL_LIBRARIES.contains(&n.as_str()))
}

impl Emu {
    /// Seed `__security_cookie` the way the kernel's image loader does.
    ///
    /// The Load Config directory names the cookie's address; the kernel
    /// writes a random value there before DriverEntry, and the driver's own
    /// `__security_init_cookie` fast-fails (code 6) if it still finds the
    /// compile-time default. A fixed value keeps runs reproducible.
    fn init_security_cookie(&mut self, image: &PE64, base: u64) {
        const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10;
        const SECURITY_COOKIE_OFFSET: u64 = 0x58;
        const COOKIE: u64 = 0x0000_4d57_454d_5521; // "MWEMU!", top 16 bits clear
        let Some(dir) = image
            .opt
            .data_directory
            .get(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG)
        else {
            return;
        };
        if dir.virtual_address == 0 || (dir.size as u64) < SECURITY_COOKIE_OFFSET + 8 {
            return;
        }
        let cookie_va = self
            .maps
            .read_qword(base + dir.virtual_address as u64 + SECURITY_COOKIE_OFFSET)
            .unwrap_or(0);
        if cookie_va == 0 {
            return;
        }
        if let Some(mem) = self.maps.get_mem_by_addr_mut(cookie_va) {
            mem.force_write_qword(cookie_va, COOKIE);
            log::debug!("security cookie seeded at 0x{:x}", cookie_va);
        }
    }

    /// Load a Windows driver and link it against the emulated kernel.
    ///
    /// Returns the image base. `DriverEntry` is *not* executed; call
    /// [`Emu::run_module_init`] for that.
    pub fn load_sys_driver(&mut self, filename: &str) -> Result<u64, MwemuError> {
        let raw = std::fs::read(filename)
            .map_err(|e| MwemuError::new(&format!("cannot read {}: {}", filename, e)))?;
        if !PE64::is_pe64(&raw) {
            return Err(MwemuError::new(&format!(
                "{} is not a PE64 image",
                filename
            )));
        }
        let image = PE64::parse(filename, &raw);
        if image.fh.machine != IMAGE_FILE_MACHINE_AMD64 {
            return Err(MwemuError::new(&format!(
                "unsupported driver architecture: machine 0x{:x} (only x64 drivers for now)",
                image.fh.machine
            )));
        }

        self.cfg.arch = Arch::X86_64;
        self.maps.is_64bits = true;
        self.filename = filename.to_string();
        self.cfg.filename = filename.to_string();

        self.kernel_init(KernelOs::Windows);
        windows::bootstrap(self);

        let base = self
            .kernel
            .as_ref()
            .ok_or_else(|| MwemuError::new("kernel env not present for .sys loading"))?
            .layout
            .module_base;
        let stem = std::path::Path::new(filename)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "driver".to_string());

        // --- map the image ---------------------------------------------------
        let layout = image.image_layout();
        let span = layout.mapped_size();
        let regions = super::pe::image_regions(&stem, &raw, layout);
        self.maps
            .map_image(base, span, &regions)
            .map_err(|e| MwemuError::new(&format!("cannot map {}: {}", filename, e)))?;
        image.apply_relocations(&raw, self, base);
        self.init_security_cookie(&image, base);

        // --- bind imports to the emulated kernel -------------------------------
        let mut imports: Vec<String> = Vec::new();
        let mut unresolved: Vec<String> = Vec::new();
        for desc in &image.image_import_descriptor {
            let library = desc.name.trim().to_string();
            let mut lookup = if desc.original_first_thunk == 0 {
                desc.first_thunk
            } else {
                desc.original_first_thunk
            };
            let mut slot = desc.first_thunk;
            if lookup == 0 || slot == 0 {
                continue;
            }
            while let Some(data) = image.image_layout().data_at(&raw, lookup) {
                if data.len() < 8 {
                    break;
                }
                let thunk = read_u64_le(data, 0);
                if thunk == 0 {
                    break;
                }
                let name = if thunk & (1 << 63) != 0 {
                    format!("{}#{}", library, thunk & 0xffff)
                } else {
                    image
                        .image_layout()
                        .data_at(&raw, thunk as u32)
                        .map(|d| read_c_string(d, 2))
                        .unwrap_or_default()
                };
                if !name.is_empty() {
                    match self.kernel_resolve_import(&name) {
                        Some(addr) => {
                            // The IAT lives in read-only .rdata: patch it the
                            // way the loader does, not the way the driver may.
                            if let Some(mem) = self.maps.get_mem_by_addr_mut(base + slot as u64) {
                                mem.force_write_qword(base + slot as u64, addr);
                            }
                        }
                        None => log::warn!("import {}!{} did not resolve", library, name),
                    }
                    if !windows::is_implemented(&name) && windows::data_symbol_size(&name).is_none()
                    {
                        unresolved.push(name.clone());
                    }
                    imports.push(name);
                }
                lookup += 8;
                slot += 8;
            }
        }
        unresolved.sort();
        unresolved.dedup();

        // --- sections and symbols -----------------------------------------------
        let sections: Vec<RelSection> = image
            .sect_hdr
            .iter()
            .enumerate()
            .map(|(i, s)| RelSection {
                index: i,
                name: s.get_name().trim().to_string(),
                addr: base + s.virtual_address as u64,
                size: s.virtual_size.max(s.size_of_raw_data) as u64,
                perm: rs_header::elf::Perm::from_flags(
                    s.characteristics & IMAGE_SCN_MEM_READ != 0,
                    s.characteristics & IMAGE_SCN_MEM_WRITE != 0,
                    s.characteristics & IMAGE_SCN_MEM_EXECUTE != 0,
                ),
            })
            .collect();
        let entry = base + image.opt.address_of_entry_point as u64;
        let mut symbols = vec![RelSymbol {
            name: "DriverEntry".to_string(),
            addr: entry,
            size: 0,
            is_func: true,
            is_global: true,
        }];
        if let Some(dir) = image.opt.data_directory.get(IMAGE_DIRECTORY_ENTRY_EXPORT)
            && let Some(parsed) =
                build_image_export_index(&raw, image.image_layout(), dir.virtual_address, dir.size)
        {
            for e in &parsed.named_exports {
                if let Some(Some(target)) = parsed.ordinal_targets.get(e.ordinal_index as usize)
                    && let rs_header::pe::export_index::ExportTarget::Direct { rva } = target
                {
                    symbols.push(RelSymbol {
                        name: e.name.clone(),
                        addr: base + *rva as u64,
                        size: 0,
                        is_func: true,
                        is_global: true,
                    });
                }
            }
        }

        log::info!(
            "loaded driver '{}' at 0x{:x} ({} bytes, {} sections, {} imports, DriverEntry 0x{:x})",
            stem,
            base,
            span,
            sections.len(),
            imports.len(),
            entry
        );
        if !unresolved.is_empty() {
            log::warn!(
                "driver imports {} symbol(s) with no kernel implementation: {}",
                unresolved.len(),
                unresolved.join(", ")
            );
        }

        let module = crate::kernel::ModuleImage {
            name: stem,
            base,
            size: span,
            init: Some(entry),
            exit: None,
            sections,
            symbols,
            unresolved,
        };
        self.kernel
            .as_mut()
            .ok_or_else(|| MwemuError::new("kernel env not present for .sys loading"))?
            .module = module;

        self.base = base;
        self.pe64 = Some(image);
        self.pe64_raw = Some(raw);
        Ok(base)
    }
}
