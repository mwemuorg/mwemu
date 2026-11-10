use crate::emu::Emu;
use crate::maps::image::ImageRegion;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::ALLOCATION_GRANULARITY;
use crate::windows::constants::memory::*;

use crate::windows::peb::{peb32, peb64};
use rs_header::pe::export_index::{ExportIndexData, build_export_index};
use rs_header::pe::image::{aligned_size, image_size, section_data, section_size};
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;
use rs_header::pe::shared::ImageSectionHeader;

/// Index of the export data directory in `data_directory[]`. Mirrors
/// `IMAGE_DIRECTORY_ENTRY_EXPORT` from `rs-header::pe::shared` (always 0).
const DATA_DIR_EXPORT: usize = 0;

const SECTION_EXECUTE: u32 = 0x2000_0000;
const SECTION_READ: u32 = 0x4000_0000;
const SECTION_WRITE: u32 = 0x8000_0000;

impl Emu {
    /// Read a PE image off disk. This is plain I/O — all interpretation of the
    /// bytes is done by `rs-header`; libmwemu never parses the raw itself.
    fn read_pe_raw(filename: &str) -> Vec<u8> {
        std::fs::read(filename).unwrap_or_else(|e| {
            log::error!("cannot read PE file {}: {}", filename, e);
            Vec::new()
        })
    }

    /// Build and register a `ModuleExportIndex` from a parsed PE's raw bytes,
    /// section table, and optional-header data directories.
    ///
    /// Safe to call on malformed / absent export directories — the parser
    /// returns `None` and we leave the registry unchanged for that module.
    fn register_export_index_from_raw(
        &mut self,
        module_name: &str,
        base: u64,
        raw: &[u8],
        sections: &[rs_header::pe::shared::ImageSectionHeader],
        export_va: u32,
        export_size: u32,
    ) {
        let Some(parsed): Option<ExportIndexData> =
            build_export_index(raw, sections, export_va, export_size)
        else {
            return;
        };
        if parsed.is_empty() {
            return;
        }
        let index = crate::api::windows::export_index::ModuleExportIndex::from_parsed(
            module_name.to_string(),
            crate::api::windows::export_index::normalize_module_name(module_name),
            base,
            &parsed,
        );
        self.export_indexes.register(index);
    }

    /// PE images use allocation granularity; other library buffers retain the
    /// page-aligned generic allocator. The caller maps the chosen range.
    pub(super) fn alloc_pe_image(&self, size: u64, bottom: u64, top: u64) -> Option<u64> {
        if size > self.maps.max_alloc_size {
            return None;
        }
        self.maps
            .find_free_range(size, ALLOCATION_GRANULARITY, bottom, top)
    }

    /// Relocatable DLLs use the library range. Preserve a non-relocatable DLL's
    /// preferred base only when the complete virtual image fits there.
    fn pick_pe64_dll_base(&self, pe64: &PE64, span: u64) -> Option<u64> {
        let base = pe64.opt.image_base;
        if base >= ALLOCATION_GRANULARITY
            && base.is_multiple_of(ALLOCATION_GRANULARITY)
            && !pe64.has_relocations()
            && base
                .checked_add(span)
                .is_some_and(|end| end <= USER64_ADDRESS_LIMIT)
            && !self.maps.overlaps(base, span)
        {
            return Some(base);
        }
        self.alloc_pe_image(span, LIBS64_MIN, LIBS64_MAX)
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE32, but also from load_library etc.
    /// cyclic stuff: [load_pe] -> [iat-binding]  ->  [load_library] -> [load_pe]
    /// Powered by rs-header's pe32 implementation.
    pub fn load_pe32(&mut self, filename: &str, set_entry: bool, force_base: u32) -> (u32, u32) {
        let is_maps = filename.contains("windows/x86/");
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let raw = Self::read_pe_raw(filename);

        if let Some(machine) = rs_header::pe::pe_machine_type(&raw)
            && machine != rs_header::pe::IMAGE_FILE_MACHINE_I386
        {
            log::warn!(
                "load_pe32: {} is not x86 (Machine=0x{:04x}), skipping",
                filename,
                machine
            );
            return (0, 0);
        }

        let mut pe32 = PE32::parse(filename, &raw);
        let mut image_span = image_size(
            pe32.opt.size_of_headers,
            pe32.opt.size_of_image,
            &pe32.sect_hdr,
            pe32.opt.section_alignment,
        );
        let is_reflective = raw
            .windows(b"ReflectiveLoader".len())
            .any(|w| w == b"ReflectiveLoader");
        if is_reflective {
            image_span = image_span.max(raw.len() as u64);
        }
        let base: u32;

        log::trace!("loading pe32 {}", filename);

        // 1. base logic

        // base is forced by libmwemu
        if force_base > 0 {
            base = force_base;

        // base is setted by user
        } else if !is_maps
            && self.cfg.code_base_addr != CFG_DEFAULT_BASE
            && !self.cfg.emulate_winapi
        {
            base = self.cfg.code_base_addr as u32;

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                if pe32.opt.image_base < ALLOCATION_GRANULARITY as u32
                    || !(pe32.opt.image_base as u64).is_multiple_of(ALLOCATION_GRANULARITY)
                    || pe32.opt.image_base as u64 + image_span > LIBS32_MIN
                    || self.maps.overlaps(pe32.opt.image_base as u64, image_span)
                {
                    base = match self.alloc_pe_image(image_span, ALLOC32_MIN, ALLOC32_MAX) {
                        Some(a) => a as u32,
                        None => {
                            log::error!("pe32 loader: out of memory");
                            return (0, 0);
                        }
                    };
                } else {
                    base = pe32.opt.image_base;
                }

            // system library
            } else {
                base = match self.alloc_pe_image(image_span, LIBS32_MIN, LIBS32_MAX) {
                    Some(a) => a as u32,
                    None => {
                        log::error!("pe32 loader: out of memory for lib");
                        return (0, 0);
                    }
                };
            }
        }

        let regions = if is_reflective {
            vec![ImageRegion {
                name: format!("{filename2}.pe"),
                offset: 0,
                size: image_span,
                permission: Permission::READ_WRITE_EXECUTE,
                data: &raw,
            }]
        } else {
            image_regions(
                &filename2,
                &raw,
                pe32.headers(&raw),
                pe32.opt.size_of_headers,
                &pe32.sect_hdr,
                pe32.opt.section_alignment,
            )
        };
        if let Err(error) = self.maps.map_image(base as u64, image_span, &regions) {
            log::error!("cannot map PE32 image {}: {}", filename, error);
            return (0, 0);
        }

        if set_entry || self.cfg.emulate_winapi {
            // 2. own this image (binding now happens after the sections are
            // mapped — rs-header's binding patches the *mapped* IAT in guest
            // memory, not the raw buffer, so the slots must exist first).
            if !is_maps || self.cfg.emulate_winapi {
                self.base = base as u64;
            }

            // 3. entry point logic
            if self.cfg.entry_point == CFG_DEFAULT_BASE {
                self.regs_mut().rip = base as u64 + pe32.opt.address_of_entry_point as u64;
                log::trace!("entry point at 0x{:x}", self.regs().rip);
            } else {
                self.regs_mut().rip = self.cfg.entry_point;
                log::trace!(
                    "entry point at 0x{:x} but forcing it at 0x{:x}",
                    base as u64 + pe32.opt.address_of_entry_point as u64,
                    self.regs().rip
                );
            }

            log::trace!("base: 0x{:x}", base);
        }

        if !is_reflective {
            pe32.apply_relocations(&raw, self, base);
        }

        // 4b'. Register the export-name index before IAT binding so that any
        // `GetProcAddress`-style lookup triggered by the binding itself can
        // resolve through host-side maps instead of rescanning the export
        // directory.
        {
            let dd = &pe32.opt.data_directory;
            let export_va = if dd.len() > DATA_DIR_EXPORT {
                dd[DATA_DIR_EXPORT].virtual_address
            } else {
                0
            };
            let export_size = if dd.len() > DATA_DIR_EXPORT {
                dd[DATA_DIR_EXPORT].size
            } else {
                0
            };
            self.register_export_index_from_raw(
                &filename2,
                base as u64,
                &raw,
                &pe32.sect_hdr,
                export_va,
                export_size,
            );
        }

        // 4c. pe binding — sections (incl. the IAT) are mapped now.
        // Reflective loaders handle their own imports via hash-based resolution.
        if !is_reflective
            && (set_entry || self.cfg.emulate_winapi)
            && (!is_maps || self.cfg.emulate_winapi)
        {
            pe32.iat_binding(&raw, self, base);
            pe32.delay_load_binding(&raw, self, base);
        }

        // 5. ldr table entry creation and link
        if set_entry {
            let _space_addr = peb32::create_ldr_entry(
                self,
                base,
                self.regs().rip as u32,
                &filename2,
                0,
                0x2c1950,
            );
            let exe_name = self.cfg.exe_name.clone();
            peb32::update_ldr_entry_base(&exe_name, base as u64, self);
        }

        // 5b. TLS callbacks — same as PE64 but with 32-bit addresses.
        if set_entry {
            let cbs = pe32.get_tls_callbacks(&raw, 0);
            if !cbs.is_empty() {
                let delta = (base as u64).wrapping_sub(pe32.opt.image_base as u64);
                self.tls_callbacks = cbs.into_iter().map(|cb| cb.wrapping_add(delta)).collect();
                log::trace!("PE32 has {} TLS callback(s)", self.tls_callbacks.len());
            }
        }

        // 5c. Implicit TLS initialization (32-bit).
        if set_entry && let Some(tls_dir) = pe32.get_tls_directory(&raw) {
            let delta = base.wrapping_sub(pe32.opt.image_base);
            let tls_index_va = tls_dir.tls_index.wrapping_add(delta) as u64;
            self.maps.write_dword(tls_index_va, 0);
            log::trace!("TLS: wrote _tls_index=0 at 0x{:x}", tls_index_va);

            let data_start = tls_dir.tls_data_start.wrapping_add(delta) as u64;
            let data_end = tls_dir.tls_data_end.wrapping_add(delta) as u64;
            let template_sz = data_end.saturating_sub(data_start) as usize;
            let total_sz = template_sz + tls_dir.zero_fill_size as usize;

            if total_sz > 0
                && let Some(tls_data_addr) = self.maps.alloc(total_sz.max(0x1000) as u64)
            {
                let _ = self.maps.create_map(
                    ".tls_data",
                    tls_data_addr,
                    total_sz.max(0x1000) as u64,
                    crate::maps::mem64::Permission::READ_WRITE,
                );
                if template_sz > 0 {
                    self.maps.memcpy(tls_data_addr, data_start, template_sz);
                }
                // TEB+0x2C -> pointer array; array[0] -> tls data
                if let Some(tls_ptrs_addr) = self.maps.alloc(0x1000) {
                    let _ = self.maps.create_map(
                        ".tls_ptrs",
                        tls_ptrs_addr,
                        0x1000,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    self.maps.write_dword(tls_ptrs_addr, tls_data_addr as u32);
                    if self.maps.exists_mapname("teb") {
                        let teb_base = self.maps.get_mem("teb").get_base();
                        self.maps.write_dword(teb_base + 0x2C, tls_ptrs_addr as u32);
                        log::trace!(
                            "TLS: TEB+0x2C=0x{:x} -> [0]=0x{:x} ({}+{} bytes)",
                            tls_ptrs_addr,
                            tls_data_addr,
                            template_sz,
                            tls_dir.zero_fill_size
                        );
                    }
                }
            }
        }

        // 6. return values
        let pe_hdr_off = pe32.dos.e_lfanew;
        if is_reflective && set_entry {
            self.pe32_flat_loaded = true;
        }
        self.pe32 = Some(pe32);
        self.pe32_raw = Some(raw);
        (base, pe_hdr_off)
    }

    pub fn map_dll_pe64(&mut self, filename: &str) -> (u64, PE64, Vec<u8>) {
        let map_name = self.filename_to_mapname(filename);
        let raw = Self::read_pe_raw(&filename.to_lowercase());
        let pe64 = PE64::parse(&filename.to_lowercase(), &raw);
        let image_span = image_size(
            pe64.opt.size_of_headers,
            pe64.opt.size_of_image,
            &pe64.sect_hdr,
            pe64.opt.section_alignment,
        );
        let Some(base) = self.pick_pe64_dll_base(&pe64, image_span) else {
            log::error!("pe64 loader: out of memory for lib");
            return (0, pe64, raw);
        };

        let regions = image_regions(
            &map_name,
            &raw,
            pe64.headers(&raw),
            pe64.opt.size_of_headers,
            &pe64.sect_hdr,
            pe64.opt.section_alignment,
        );
        if let Err(error) = self.maps.map_image(base, image_span, &regions) {
            log::error!("cannot map PE64 DLL {}: {}", filename, error);
            return (0, pe64, raw);
        }

        pe64.apply_relocations(&raw, self, base);

        // Register the export-name index for this mapped DLL. The normal 64-bit
        // init path binds IATs in a later loop using temporary `Lib` values,
        // so the registry must already contain every mapped DLL at that point.
        {
            let dd = &pe64.opt.data_directory;
            let export_va = if dd.len() > DATA_DIR_EXPORT {
                dd[DATA_DIR_EXPORT].virtual_address
            } else {
                0
            };
            let export_size = if dd.len() > DATA_DIR_EXPORT {
                dd[DATA_DIR_EXPORT].size
            } else {
                0
            };
            self.register_export_index_from_raw(
                &map_name,
                base,
                &raw,
                &pe64.sect_hdr,
                export_va,
                export_size,
            );
        }

        (base, pe64, raw)
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE64, but also from load_library etc.
    /// cyclic stuff: [load_pe] -> [iat-binding]  ->  [load_library] -> [load_pe]
    /// Powered by rs-header's pe64 implementation.
    pub fn load_pe64(&mut self, filename: &str, set_entry: bool, force_base: u64) -> (u64, u32) {
        let is_maps = filename.contains("windows/x86_64/") || filename.contains("windows/aarch64/");
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let raw = Self::read_pe_raw(filename);

        if let Some(machine) = rs_header::pe::pe_machine_type(&raw)
            && machine == rs_header::pe::IMAGE_FILE_MACHINE_I386
        {
            log::warn!(
                "load_pe64: {} is x86 (Machine=0x{:04x}), skipping",
                filename,
                machine
            );
            return (0, 0);
        }

        let mut pe64 = PE64::parse(filename, &raw);
        let image_span = image_size(
            pe64.opt.size_of_headers,
            pe64.opt.size_of_image,
            &pe64.sect_hdr,
            pe64.opt.section_alignment,
        );
        let base: u64;

        // 1. base logic

        // base is setted by libmwemu
        if force_base > 0 {
            base = force_base;

        // base is setted by user
        } else if !is_maps && self.cfg.code_base_addr != CFG_DEFAULT_BASE {
            base = self.cfg.code_base_addr;

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                let preferred = pe64.opt.image_base;
                let preferred_usable = if let Some(end) = preferred.checked_add(image_span) {
                    (ALLOCATION_GRANULARITY..LIBS64_MIN).contains(&preferred)
                        && preferred.is_multiple_of(ALLOCATION_GRANULARITY)
                        && end <= LIBS64_MIN
                        && !self.maps.overlaps(preferred, image_span)
                } else {
                    false
                };

                if preferred_usable {
                    base = preferred;
                } else {
                    log::trace!(
                        "pe64: preferred image base 0x{:x} unavailable or outside program range, relocating",
                        preferred
                    );
                    base = match self.alloc_pe_image(image_span, ALLOC64_MIN, ALLOC64_MAX) {
                        Some(a) => a,
                        None => {
                            log::error!("pe64 loader: out of memory");
                            return (0, 0);
                        }
                    };
                }

            // system library
            } else {
                base = match self.pick_pe64_dll_base(&pe64, image_span) {
                    Some(base) => base,
                    None => {
                        log::error!("pe64 loader: out of memory for lib");
                        return (0, 0);
                    }
                };
            }
        }

        let regions = image_regions(
            &filename2,
            &raw,
            pe64.headers(&raw),
            pe64.opt.size_of_headers,
            &pe64.sect_hdr,
            pe64.opt.section_alignment,
        );
        if let Err(error) = self.maps.map_image(base, image_span, &regions) {
            log::error!("cannot map PE64 image {}: {}", filename, error);
            return (0, 0);
        }

        // Only the main image owns `self.base` and the initial RIP. System DLLs loaded
        // via `load_library` (e.g. NtMapViewOfSection's KnownDll path under --ssdt) must
        // never clobber these — otherwise the post-LdrInitializeThunk IAT binding in
        // loaders.rs reads the DLL's base instead of the EXE's, and the EXE's IAT stays
        // unbound (call rax → rax=0 → crash).
        if set_entry {
            self.base = base;

            // 2. entry point logic (relocs + IAT run after PE maps exist; see step 4b below)
            if self.cfg.entry_point == CFG_DEFAULT_BASE {
                self.set_pc(base + pe64.opt.address_of_entry_point as u64);
                log::trace!("entry point at 0x{:x}", self.pc());
            } else {
                self.set_pc(self.cfg.entry_point);
                log::trace!(
                    "entry point at 0x{:x} but forcing it at 0x{:x} by -a flag",
                    base + pe64.opt.address_of_entry_point as u64,
                    self.pc()
                );
            }
            log::trace!("base: 0x{:x}", base);
        }

        // 4b. Base relocs on the mapped image (all load paths, including DLL without emulate_winapi).
        pe64.apply_relocations(&raw, self, base);

        // 4b'. Register the export-name index before any IAT binding can
        // resolve an export from this image. Recursively-loaded DLLs and the
        // main image both pass through this point.
        {
            let dd = &pe64.opt.data_directory;
            let export_va = if dd.len() > DATA_DIR_EXPORT {
                dd[DATA_DIR_EXPORT].virtual_address
            } else {
                0
            };
            let export_size = if dd.len() > DATA_DIR_EXPORT {
                dd[DATA_DIR_EXPORT].size
            } else {
                0
            };
            self.register_export_index_from_raw(
                &filename2,
                base,
                &raw,
                &pe64.sect_hdr,
                export_va,
                export_size,
            );
        }

        // Decide whether to eagerly bind this image's IAT.
        let bind_iat = if self.cfg.emulate_winapi {
            // SSDT/syscall mode: the main image's IAT is bound later by the
            // LdrInitializeThunk bootstrap, so skip it here; bind everything else.
            !set_entry
        } else {
            // Default API-stub mode: bind the main image AND every dependency DLL
            // loaded from maps/. mwemu executes the *real* bytes of any exported
            // function it has no stub for (e.g. msvcrt's CRT startup `_initterm`,
            // `__wgetmainargs`), and that real code calls further APIs through this
            // image's own IAT. Pointing those slots at the real export VAs is safe:
            // mwemu still intercepts them via its API gateway. Without this, the
            // dependency's IAT stays unbound and the first such call jumps to the
            // raw import-name RVA and crashes.
            true
        };
        if bind_iat {
            pe64.iat_binding(&raw, self, base);
            pe64.delay_load_binding(&raw, self, base);
        }

        // 5. ldr table entry creation and link
        if set_entry {
            if !self.cfg.emulate_winapi {
                let _space_addr =
                    peb64::create_ldr_entry(self, base, self.pc(), &filename2, 0, 0x2c1950);
                let exe_name = self.cfg.exe_name.clone();
                peb64::update_ldr_entry_base(&exe_name, base, self);
            }
            if self.cfg.emulate_winapi {
                peb64::update_peb_image_base(self, base);
            }
        }

        // 5b. TLS callbacks — extract and rebase to the load address so the
        // engine can run them before the entry point (DLL_PROCESS_ATTACH). Only
        // for the main image; a DLL's callbacks shouldn't overwrite the EXE's.
        if set_entry {
            let cbs = pe64.get_tls_callbacks(&raw, 0);
            if !cbs.is_empty() {
                let delta = base.wrapping_sub(pe64.opt.image_base);
                self.tls_callbacks = cbs.into_iter().map(|cb| cb.wrapping_add(delta)).collect();
                log::trace!("PE has {} TLS callback(s)", self.tls_callbacks.len());
            }
        }

        // 5c. Implicit TLS initialization — write _tls_index, copy template
        // data, and set up TEB.ThreadLocalStoragePointer so __declspec(thread)
        // variables work (critical for MinGW CRT).
        if set_entry && let Some(tls_dir) = pe64.get_tls_directory(&raw) {
            let delta = base.wrapping_sub(pe64.opt.image_base);
            let tls_index_va = tls_dir.tls_index.wrapping_add(delta);
            self.maps.write_dword(tls_index_va, 0);
            log::trace!("TLS: wrote _tls_index=0 at 0x{:x}", tls_index_va);

            let data_start = tls_dir.tls_data_start.wrapping_add(delta);
            let data_end = tls_dir.tls_data_end.wrapping_add(delta);
            let template_sz = data_end.saturating_sub(data_start) as usize;
            let total_sz = template_sz + tls_dir.zero_fill_size as usize;

            if total_sz > 0
                && let Some(tls_data_addr) = self.maps.alloc(total_sz.max(0x1000) as u64)
            {
                let _ = self.maps.create_map(
                    ".tls_data",
                    tls_data_addr,
                    total_sz.max(0x1000) as u64,
                    crate::maps::mem64::Permission::READ_WRITE,
                );
                if template_sz > 0 {
                    self.maps.memcpy(tls_data_addr, data_start, template_sz);
                }
                // TEB+0x58 -> pointer array; array[0] -> tls data
                if let Some(tls_ptrs_addr) = self.maps.alloc(0x1000) {
                    let _ = self.maps.create_map(
                        ".tls_ptrs",
                        tls_ptrs_addr,
                        0x1000,
                        crate::maps::mem64::Permission::READ_WRITE,
                    );
                    self.maps.write_qword(tls_ptrs_addr, tls_data_addr);
                    if self.maps.exists_mapname("teb") {
                        let teb_base = self.maps.get_mem("teb").get_base();
                        self.maps.write_qword(teb_base + 0x58, tls_ptrs_addr);
                        log::trace!(
                            "TLS: TEB+0x58=0x{:x} -> [0]=0x{:x} ({}+{} bytes)",
                            tls_ptrs_addr,
                            tls_data_addr,
                            template_sz,
                            tls_dir.zero_fill_size
                        );
                    }
                }
            }
        }

        // 6. return values
        let pe_hdr_off = pe64.dos.e_lfanew;
        self.pe64 = Some(pe64);
        self.pe64_raw = Some(raw);
        (base, pe_hdr_off)
    }
}

fn image_regions<'a>(
    module: &str,
    raw: &'a [u8],
    headers: &'a [u8],
    header_size: u32,
    sections: &[ImageSectionHeader],
    alignment: u32,
) -> Vec<ImageRegion<'a>> {
    let mut regions = vec![ImageRegion {
        name: format!("{module}.pe"),
        offset: 0,
        size: aligned_size(header_size, alignment),
        permission: Permission::READ_WRITE,
        data: headers,
    }];
    for section in sections {
        let size = section_size(section, alignment);
        if size == 0 {
            continue;
        }
        let original_name = section.get_name();
        let section_name = original_name.replace([' ', '\t', '\n', '\r'], "");
        let section_name = if section_name.is_empty() {
            format!("{:x}", section.virtual_address)
        } else {
            section_name
        };
        let mut name = format!("{module}{section_name}");
        if regions.iter().any(|region| region.name == name) {
            name = format!("{name}_{:x}", section.virtual_address);
        }
        // Windows init patches .didat before calling VirtualProtect; keep the
        // existing writable approximation until image COW is implemented.
        let permission = Permission::from_flags(
            section.characteristics & SECTION_READ != 0,
            section.characteristics & SECTION_WRITE != 0 || original_name.trim() == ".didat",
            section.characteristics & SECTION_EXECUTE != 0,
        );
        regions.push(ImageRegion {
            name,
            offset: section.virtual_address as u64,
            size,
            permission,
            data: section_data(raw, section, alignment),
        });
    }
    regions
}
