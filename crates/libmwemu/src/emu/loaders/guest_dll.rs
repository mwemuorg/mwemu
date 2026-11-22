use goblin::pe::PE;
use rs_header::pe::image::aligned_size;
use rs_header::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386};
use serde::{Deserialize, Serialize};

use super::pe::PeLoadError;
use crate::api::windows::export_index::normalize_module_name;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::winapi::policy::invoke_callback;
use crate::windows::constants::errors::*;
use crate::windows::constants::process::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use crate::windows::peb::{peb32, peb64, unlink_dynamic_entry};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct GuestDll {
    path: String,
    name: String,
    span: u64,
    entry: u64,
    ldr: u64,
    references: u32,
    initializing: bool,
}

struct GuestImageLayout {
    span: u64,
    entry_rva: u64,
    header_offset: u32,
}

impl Emu {
    pub(crate) fn guest_dll_base(&self, path: &str) -> Option<u64> {
        let key = path.replace('/', "\\").to_ascii_lowercase();
        let qualified = key.contains('\\') || key.contains(':');
        let name = normalize_module_name(path);
        self.guest_dlls.iter().find_map(|(&base, dll)| {
            (if qualified {
                dll.path == key
            } else {
                dll.name == name
            })
            .then_some(base)
        })
    }

    pub(crate) fn retain_guest_dll(&mut self, path: &str) -> Option<u64> {
        let base = self.guest_dll_base(path)?;
        let dll = self.guest_dlls.get_mut(&base)?;
        dll.references = dll.references.checked_add(1)?;
        Some(base)
    }

    pub(crate) fn is_guest_dll_address(&self, addr: u64) -> bool {
        self.guest_dlls
            .range(..=addr)
            .next_back()
            .is_some_and(|(&base, dll)| addr - base < dll.span)
    }

    /// Loads file-layout bytes into guest memory. All entry points execute through the emulator.
    pub(crate) fn load_guest_dll(&mut self, path: &str, raw: Vec<u8>) -> Result<u64, PeLoadError> {
        if let Some(base) = self.retain_guest_dll(path) {
            return Ok(base);
        }
        let name = normalize_module_name(path);
        if name.is_empty()
            || self.export_indexes.get_by_name(&name).is_some()
            || self.guest_dlls.values().any(|dll| dll.name == name)
        {
            return Err(PeLoadError::new(
                ERROR_BAD_EXE_FORMAT,
                "conflicting DLL name",
            ));
        }
        let GuestImageLayout {
            span,
            entry_rva,
            header_offset,
        } = self.guest_image_layout(path, &raw)?;
        let base = self.map_guest_pe(&format!("{name}.dll"), raw)?;
        let entry = if entry_rva == 0 { 0 } else { base + entry_rva };
        if entry != 0 && !self.maps.can_access_range(entry, 1, Permission::EXECUTE) {
            self.maps.unmap_range(base, span);
            self.export_indexes.remove(&name);
            return Err(PeLoadError::new(
                ERROR_BAD_EXE_FORMAT,
                "guest DLL entry point is not executable",
            ));
        }
        let ldr = if self.maps.get_map_by_name("peb").is_none() {
            0
        } else if self.cfg.is_x64() {
            peb64::link_dynamic_entry(base, header_offset, path, self)
        } else {
            peb32::link_dynamic_entry(base, entry as u32, path, self)
        };
        self.guest_dlls.insert(
            base,
            GuestDll {
                path: path.replace('/', "\\").to_ascii_lowercase(),
                name,
                span,
                entry,
                ldr,
                references: 1,
                initializing: true,
            },
        );
        if entry != 0 {
            match invoke_callback(
                self,
                path,
                "DllMain",
                entry,
                [base, DLL_PROCESS_ATTACH as u64, 0],
            ) {
                Some(value) if value as u32 == 0 => {
                    if invoke_callback(
                        self,
                        path,
                        "DllMain",
                        entry,
                        [base, DLL_PROCESS_DETACH as u64, 0],
                    )
                    .is_some()
                    {
                        self.remove_guest_dll(base);
                    }
                    return Err(PeLoadError::new(
                        ERROR_DLL_INIT_FAILED,
                        "DllMain rejected process attach",
                    ));
                }
                None => {
                    return Err(PeLoadError::new(
                        ERROR_DLL_INIT_FAILED,
                        "guest stopped in DllMain",
                    ));
                }
                _ => {}
            }
        }
        if let Some(dll) = self.guest_dlls.get_mut(&base) {
            dll.initializing = false;
        }
        self.library_loaded = true;
        Ok(base)
    }

    fn guest_image_layout(&self, path: &str, raw: &[u8]) -> Result<GuestImageLayout, PeLoadError> {
        let pe = PE::parse(raw).map_err(|error| {
            PeLoadError::new(
                ERROR_BAD_EXE_FORMAT,
                format!("invalid guest DLL '{path}': {error}"),
            )
        })?;
        let machine = match self.cfg.arch {
            Arch::X86 => IMAGE_FILE_MACHINE_I386,
            Arch::X86_64 => IMAGE_FILE_MACHINE_AMD64,
            Arch::Aarch64 => {
                return Err(PeLoadError::new(
                    ERROR_BAD_EXE_FORMAT,
                    "unsupported guest DLL architecture",
                ));
            }
        };
        if !self.os.is_windows()
            || !pe.is_lib
            || pe.header.coff_header.machine != machine
            || pe.is_64 != self.cfg.is_x64()
        {
            return Err(PeLoadError::new(
                ERROR_BAD_EXE_FORMAT,
                "DLL does not match the guest architecture",
            ));
        }
        let opt = pe
            .header
            .optional_header
            .ok_or_else(|| PeLoadError::new(ERROR_BAD_EXE_FORMAT, "missing DLL optional header"))?;
        if opt.data_directories.get_tls_table().is_some() {
            return Err(PeLoadError::new(
                ERROR_NOT_SUPPORTED,
                "dynamic DLL TLS is not modeled",
            ));
        }
        let alignment = opt.windows_fields.section_alignment;
        let declared = u64::from(opt.windows_fields.size_of_image);
        let span = if alignment.is_power_of_two() && u64::from(alignment) < PAGE_SIZE {
            declared.next_multiple_of(PAGE_SIZE)
        } else {
            declared
        };
        if aligned_size(opt.windows_fields.size_of_headers, alignment) > span
            || pe.sections.iter().any(|section| {
                let size = if section.virtual_size == 0 {
                    section.size_of_raw_data
                } else {
                    section.virtual_size
                };
                u64::from(section.virtual_address) + aligned_size(size, alignment) > span
            })
        {
            return Err(PeLoadError::new(
                ERROR_BAD_EXE_FORMAT,
                "section exceeds declared DLL image size",
            ));
        }
        if span == 0 || span > self.maps.max_alloc_size || pe.entry as u64 >= span {
            return Err(PeLoadError::new(
                ERROR_BAD_EXE_FORMAT,
                "invalid guest DLL image or entry point",
            ));
        }
        Ok(GuestImageLayout {
            span,
            entry_rva: pe.entry as u64,
            header_offset: pe.header.dos_header.pe_pointer,
        })
    }

    pub(crate) fn release_guest_dll(&mut self, base: u64) -> Option<bool> {
        let dll = self.guest_dlls.get_mut(&base)?;
        if dll.references > 1 {
            dll.references -= 1;
            return Some(true);
        }
        if dll.initializing {
            return Some(false);
        }
        dll.initializing = true;
        let entry = dll.entry;
        if entry != 0
            && invoke_callback(
                self,
                "loader",
                "DllMain",
                entry,
                [base, DLL_PROCESS_DETACH as u64, 0],
            )
            .is_none()
        {
            return Some(false);
        }
        self.remove_guest_dll(base);
        Some(true)
    }

    fn remove_guest_dll(&mut self, base: u64) {
        if let Some(dll) = self.guest_dlls.remove(&base) {
            if dll.ldr != 0 {
                unlink_dynamic_entry(self, dll.ldr);
            }
            self.maps.unmap_range(base, dll.span);
            self.export_indexes.remove(&dll.name);
            self.api_resolve_cache.clear();
            self.api_addr_name_cache.clear();
            self.library_loaded = true;
        }
    }
}
