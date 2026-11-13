use goblin::pe::section_table::{IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE};

use crate::api::windows::export_index::{ModuleExportIndex, normalize_module_name};
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::image::ImageRegion;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, PAGE_SIZE};
use crate::windows::constants::memory::*;
use crate::windows::peb::{peb32, peb64};
use crate::windows::structures::{TEB, TEB64};
use rs_header::pe::export_index::build_image_export_index;
use rs_header::pe::image::{ImageLayout, aligned_size, section_data, section_size};
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;
use rs_header::pe::shared::{IMAGE_DIRECTORY_ENTRY_EXPORT, ImageDataDirectory};

const INITIAL_LDR_PREVIOUS_LINK: u32 = 0x2c1950;

/// Location of a successfully loaded PE image in guest memory.
#[derive(Debug, Clone, Copy)]
pub struct PeLoadInfo {
    pub base: u64,
    pub header_offset: u32,
}

/// A relocated PE64 image whose imports can be bound after mapping dependencies.
pub struct MappedPe64 {
    pub base: u64,
    pub image: PE64,
    pub raw: Vec<u8>,
}

impl Emu {
    fn read_pe_raw(filename: &str) -> Result<Vec<u8>, MwemuError> {
        std::fs::read(filename)
            .map_err(|error| MwemuError::new(&format!("cannot read PE file '{filename}': {error}")))
    }

    // Register exports before binding imports, including recursively loaded DLLs.
    // Absent or malformed export directories leave the registry unchanged.
    fn register_pe_exports(
        &mut self,
        module_name: &str,
        base: u64,
        raw: &[u8],
        layout: ImageLayout<'_>,
        directories: &[ImageDataDirectory],
    ) {
        let Some(directory) = directories.get(IMAGE_DIRECTORY_ENTRY_EXPORT) else {
            return;
        };
        let Some(parsed) =
            build_image_export_index(raw, layout, directory.virtual_address, directory.size)
        else {
            return;
        };
        if !parsed.is_empty() {
            self.export_indexes.register(ModuleExportIndex::from_parsed(
                module_name.to_string(),
                normalize_module_name(module_name),
                base,
                &parsed,
            ));
        }
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

    fn pe_base_available(&self, base: u64, span: u64, limit: u64) -> bool {
        base >= ALLOCATION_GRANULARITY
            && base.is_multiple_of(ALLOCATION_GRANULARITY)
            && base.checked_add(span).is_some_and(|end| end <= limit)
            && !self.maps.overlaps(base, span)
    }

    fn pick_pe32_base(&self, pe: &PE32, span: u64, main_image: bool) -> Option<u64> {
        if !main_image {
            return self.alloc_pe_image(span, LIBS32_MIN, LIBS32_MAX);
        }
        let preferred = pe.opt.image_base as u64;
        if self.pe_base_available(preferred, span, LIBS32_MIN) {
            Some(preferred)
        } else {
            self.alloc_pe_image(span, ALLOC32_MIN, ALLOC32_MAX)
        }
    }

    fn pick_pe64_base(&self, pe: &PE64, span: u64, main_image: bool) -> Option<u64> {
        let preferred = pe.opt.image_base;
        let limit = if main_image {
            LIBS64_MIN
        } else {
            USER64_ADDRESS_LIMIT
        };
        // Relocatable DLLs use the library range; preserve a fixed DLL's base.
        if (main_image || !pe.has_relocations()) && self.pe_base_available(preferred, span, limit) {
            return Some(preferred);
        }
        if main_image {
            self.alloc_pe_image(span, ALLOC64_MIN, ALLOC64_MAX)
        } else {
            self.alloc_pe_image(span, LIBS64_MIN, LIBS64_MAX)
        }
    }

    fn set_pe_entry_point(&mut self, base: u64, entry_rva: u32) {
        self.base = base;
        let entry = base + entry_rva as u64;
        let actual = if self.cfg.entry_point == CFG_DEFAULT_BASE {
            entry
        } else {
            self.cfg.entry_point
        };
        self.set_pc(actual);
        log::trace!("PE base: 0x{base:x}, entry point: 0x{entry:x}, initial PC: 0x{actual:x}");
    }

    /// Load and bind a PE32 image. Only the main image owns process metadata,
    /// the initial instruction pointer and TLS state.
    pub fn load_pe32(
        &mut self,
        filename: &str,
        main_image: bool,
        force_base: u32,
    ) -> Result<PeLoadInfo, MwemuError> {
        let is_maps = filename.contains("windows/x86/");
        let map_name = self.filename_to_mapname(filename);
        let raw = Self::read_pe_raw(filename)?;
        if !PE32::is_pe32(&raw) {
            return Err(MwemuError::new(&format!(
                "'{filename}' is not a PE32 image"
            )));
        }
        let mut pe32 = PE32::parse(filename, &raw);
        let mut image_span = pe32.image_layout().mapped_size();
        let is_reflective = raw
            .windows(b"ReflectiveLoader".len())
            .any(|w| w == b"ReflectiveLoader");
        if is_reflective {
            image_span = image_span.max(raw.len() as u64);
        }
        let base = if force_base != 0 {
            force_base
        } else if main_image
            && !is_maps
            && self.cfg.code_base_addr != CFG_DEFAULT_BASE
            && !self.cfg.emulate_winapi
        {
            self.cfg.code_base_addr as u32
        } else {
            self.pick_pe32_base(&pe32, image_span, main_image)
                .ok_or_else(|| {
                    MwemuError::new(&format!(
                        "cannot allocate PE32 image '{filename}' ({image_span:#x} bytes)"
                    ))
                })? as u32
        };
        let regions = if is_reflective {
            vec![ImageRegion {
                name: format!("{map_name}.pe"),
                offset: 0,
                size: image_span,
                permission: Permission::READ_WRITE_EXECUTE,
                data: &raw,
            }]
        } else {
            image_regions(&map_name, &raw, pe32.image_layout())
        };
        self.maps
            .map_image(base as u64, image_span, &regions)
            .map_err(|error| {
                MwemuError::new(&format!("cannot map PE32 image '{filename}': {error}"))
            })?;

        if main_image {
            self.set_pe_entry_point(base as u64, pe32.opt.address_of_entry_point);
            if self.maps.get_map_by_name("peb").is_some() {
                peb32::update_peb_image_base(self, base);
            }
        }
        if !is_reflective {
            pe32.apply_relocations(&raw, self, base);
        }
        self.register_pe_exports(
            &map_name,
            base as u64,
            &raw,
            pe32.image_layout(),
            &pe32.opt.data_directory,
        );

        // Reflective loaders resolve their own imports.
        if !is_reflective
            && (main_image || self.cfg.emulate_winapi)
            && (!is_maps || self.cfg.emulate_winapi)
        {
            pe32.iat_binding(&raw, self, base);
            pe32.delay_load_binding(&raw, self, base);
        }
        let info = PeLoadInfo {
            base: base as u64,
            header_offset: pe32.dos.e_lfanew,
        };
        if main_image {
            peb32::create_ldr_entry(
                self,
                base,
                self.pc() as u32,
                &map_name,
                0,
                INITIAL_LDR_PREVIOUS_LINK,
            );
            let exe_name = self.cfg.exe_name.clone();
            peb32::update_ldr_entry_base(&exe_name, base as u64, self);
            self.initialize_pe32_tls(&pe32, &raw, base);
            self.pe32_flat_loaded = is_reflective;
            self.pe32 = Some(pe32);
            self.pe32_raw = Some(raw);
        }
        Ok(info)
    }

    /// Map and relocate a PE64 DLL without binding its imports.
    pub fn map_dll_pe64(&mut self, filename: &str) -> Result<MappedPe64, MwemuError> {
        self.map_pe64_image(filename, false, 0)
    }

    fn map_pe64_image(
        &mut self,
        filename: &str,
        main_image: bool,
        force_base: u64,
    ) -> Result<MappedPe64, MwemuError> {
        let raw = Self::read_pe_raw(filename)?;
        if !PE64::is_pe64(&raw) {
            return Err(MwemuError::new(&format!(
                "'{filename}' is not a PE64 image"
            )));
        }
        let image = PE64::parse(filename, &raw);
        let image_span = image.image_layout().mapped_size();
        let is_maps = filename.contains("windows/x86_64/") || filename.contains("windows/aarch64/");
        let base = if force_base != 0 {
            force_base
        } else if main_image && !is_maps && self.cfg.code_base_addr != CFG_DEFAULT_BASE {
            self.cfg.code_base_addr
        } else {
            self.pick_pe64_base(&image, image_span, main_image)
                .ok_or_else(|| {
                    MwemuError::new(&format!(
                        "cannot allocate PE64 image '{filename}' ({image_span:#x} bytes)"
                    ))
                })?
        };
        let map_name = self.filename_to_mapname(filename);
        let regions = image_regions(&map_name, &raw, image.image_layout());
        self.maps
            .map_image(base, image_span, &regions)
            .map_err(|error| {
                MwemuError::new(&format!("cannot map PE64 image '{filename}': {error}"))
            })?;
        image.apply_relocations(&raw, self, base);
        self.register_pe_exports(
            &map_name,
            base,
            &raw,
            image.image_layout(),
            &image.opt.data_directory,
        );
        Ok(MappedPe64 { base, image, raw })
    }

    /// Load and bind a PE64 image. Dependency loading preserves main-image state.
    pub fn load_pe64(
        &mut self,
        filename: &str,
        main_image: bool,
        force_base: u64,
    ) -> Result<PeLoadInfo, MwemuError> {
        let MappedPe64 {
            base,
            mut image,
            raw,
        } = self.map_pe64_image(filename, main_image, force_base)?;
        if main_image {
            self.set_pe_entry_point(base, image.opt.address_of_entry_point);
            if self.maps.get_map_by_name("peb").is_some() {
                peb64::update_peb_image_base(self, base);
            }
        }
        // SSDT mode defers the main image's imports to LdrInitializeThunk.
        // Dependencies need bound imports even when API stubs handle their exports.
        if !self.cfg.emulate_winapi || !main_image {
            image.iat_binding(&raw, self, base);
            image.delay_load_binding(&raw, self, base);
        }
        let info = PeLoadInfo {
            base,
            header_offset: image.dos.e_lfanew,
        };
        if main_image {
            if !self.cfg.emulate_winapi {
                let map_name = self.filename_to_mapname(filename);
                peb64::create_ldr_entry(
                    self,
                    base,
                    self.pc(),
                    &map_name,
                    0,
                    INITIAL_LDR_PREVIOUS_LINK as _,
                );
                let exe_name = self.cfg.exe_name.clone();
                peb64::update_ldr_entry_base(&exe_name, base, self);
            }
            self.initialize_pe64_tls(&image, &raw, base);
            self.pe64 = Some(image);
            self.pe64_raw = Some(raw);
        }
        Ok(info)
    }

    fn initialize_pe32_tls(&mut self, pe32: &PE32, raw: &[u8], base: u32) {
        let delta = (base as u64).wrapping_sub(pe32.opt.image_base as u64);
        self.tls_callbacks = pe32
            .get_tls_callbacks(raw, 0)
            .into_iter()
            .map(|cb| cb.wrapping_add(delta))
            .collect();
        log::trace!("PE32 has {} TLS callback(s)", self.tls_callbacks.len());
        if let Some(tls_dir) = pe32.get_tls_directory(raw) {
            let delta = base.wrapping_sub(pe32.opt.image_base);
            let tls_index_va = tls_dir.tls_index.wrapping_add(delta) as u64;
            self.maps.write_dword(tls_index_va, 0);
            log::trace!("TLS: wrote _tls_index=0 at 0x{:x}", tls_index_va);

            let data_start = tls_dir.tls_data_start.wrapping_add(delta) as u64;
            let data_end = tls_dir.tls_data_end.wrapping_add(delta) as u64;
            let template_sz = data_end.saturating_sub(data_start) as usize;
            let total_sz = template_sz + tls_dir.zero_fill_size as usize;

            if total_sz > 0
                && let Some(tls_data_addr) = self.maps.alloc((total_sz as u64).max(PAGE_SIZE))
            {
                let _ = self.maps.create_map(
                    ".tls_data",
                    tls_data_addr,
                    (total_sz as u64).max(PAGE_SIZE),
                    Permission::READ_WRITE,
                );
                if template_sz > 0 {
                    self.maps.memcpy(tls_data_addr, data_start, template_sz);
                }
                // TEB+0x2C -> pointer array; array[0] -> tls data
                if let Some(tls_ptrs_addr) = self.maps.alloc(PAGE_SIZE) {
                    let _ = self.maps.create_map(
                        ".tls_ptrs",
                        tls_ptrs_addr,
                        PAGE_SIZE,
                        Permission::READ_WRITE,
                    );
                    self.maps.write_dword(tls_ptrs_addr, tls_data_addr as u32);
                    if self.maps.exists_mapname("teb") {
                        let teb_base = self.maps.get_mem("teb").get_base();
                        self.maps.write_dword(
                            teb_base + TEB::THREAD_LOCAL_STORAGE_POINTER_OFFSET,
                            tls_ptrs_addr as u32,
                        );
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
    }

    fn initialize_pe64_tls(&mut self, pe64: &PE64, raw: &[u8], base: u64) {
        let delta = base.wrapping_sub(pe64.opt.image_base);
        self.tls_callbacks = pe64
            .get_tls_callbacks(raw, 0)
            .into_iter()
            .map(|cb| cb.wrapping_add(delta))
            .collect();
        log::trace!("PE64 has {} TLS callback(s)", self.tls_callbacks.len());
        if let Some(tls_dir) = pe64.get_tls_directory(raw) {
            let delta = base.wrapping_sub(pe64.opt.image_base);
            let tls_index_va = tls_dir.tls_index.wrapping_add(delta);
            self.maps.write_dword(tls_index_va, 0);
            log::trace!("TLS: wrote _tls_index=0 at 0x{:x}", tls_index_va);

            let data_start = tls_dir.tls_data_start.wrapping_add(delta);
            let data_end = tls_dir.tls_data_end.wrapping_add(delta);
            let template_sz = data_end.saturating_sub(data_start) as usize;
            let total_sz = template_sz + tls_dir.zero_fill_size as usize;

            if total_sz > 0
                && let Some(tls_data_addr) = self.maps.alloc((total_sz as u64).max(PAGE_SIZE))
            {
                let _ = self.maps.create_map(
                    ".tls_data",
                    tls_data_addr,
                    (total_sz as u64).max(PAGE_SIZE),
                    Permission::READ_WRITE,
                );
                if template_sz > 0 {
                    self.maps.memcpy(tls_data_addr, data_start, template_sz);
                }
                // TEB+0x58 -> pointer array; array[0] -> tls data
                if let Some(tls_ptrs_addr) = self.maps.alloc(PAGE_SIZE) {
                    let _ = self.maps.create_map(
                        ".tls_ptrs",
                        tls_ptrs_addr,
                        PAGE_SIZE,
                        Permission::READ_WRITE,
                    );
                    self.maps.write_qword(tls_ptrs_addr, tls_data_addr);
                    if self.maps.exists_mapname("teb") {
                        let teb_base = self.maps.get_mem("teb").get_base();
                        self.maps.write_qword(
                            teb_base + TEB64::THREAD_LOCAL_STORAGE_POINTER_OFFSET,
                            tls_ptrs_addr,
                        );
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
    }
}

pub(crate) fn image_regions<'a>(
    module: &str,
    raw: &'a [u8],
    layout: ImageLayout<'_>,
) -> Vec<ImageRegion<'a>> {
    if layout.is_flat() {
        // Windows maps low-alignment images as one executable copy-on-write
        // view. Use the existing writable approximation until image COW exists.
        return vec![ImageRegion {
            name: format!("{module}.pe"),
            offset: 0,
            size: layout.mapped_size(),
            permission: Permission::READ_WRITE_EXECUTE,
            data: &raw[..raw.len().min(layout.size as usize)],
        }];
    }
    let mut regions = vec![ImageRegion {
        name: format!("{module}.pe"),
        offset: 0,
        size: aligned_size(layout.headers, layout.alignment),
        permission: Permission::READ_WRITE,
        data: &raw[..raw.len().min(layout.headers as usize)],
    }];
    for section in layout.sections {
        let size = section_size(section, layout.alignment);
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
            section.characteristics & IMAGE_SCN_MEM_READ != 0,
            section.characteristics & IMAGE_SCN_MEM_WRITE != 0 || original_name.trim() == ".didat",
            section.characteristics & IMAGE_SCN_MEM_EXECUTE != 0,
        );
        regions.push(ImageRegion {
            name,
            offset: section.virtual_address as u64,
            size,
            permission,
            data: section_data(raw, section, layout.alignment),
        });
    }
    regions
}
