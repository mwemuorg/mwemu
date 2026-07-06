use crate::pe::readers::{
    read_u16_le as read_u16_le_shared, read_u32_le as read_u32_le_shared,
    read_u64_le as read_u64_le_shared, read_u8 as read_u8_shared,
};

use super::{
    DelayLoadDirectory, ImageDataDirectory, ImageDosHeader, ImageFileHeader, ImageImportDescriptor,
    ImageNtHeaders, ImageSectionHeader, IMAGE_NUMBEROF_DIRECTORY_ENTRIES,
};

macro_rules! read_u8 {
    ($raw:expr, $off:expr) => {
        read_u8_shared(($raw).as_ref(), $off)
    };
}

macro_rules! read_u16_le {
    ($raw:expr, $off:expr) => {
        read_u16_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

pub(crate) const IMAGE_FILE_DLL: u16 = 0x2000;

#[derive(Debug)]
pub struct ImageOptionalHeader64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: Vec<ImageDataDirectory>,
}

impl ImageOptionalHeader64 {
    pub fn load(raw: &[u8], off: usize) -> ImageOptionalHeader64 {
        let mut dd: Vec<ImageDataDirectory> = Vec::new();
        let mut pos = 112;
        for _ in 0..IMAGE_NUMBEROF_DIRECTORY_ENTRIES {
            dd.push(ImageDataDirectory::load(raw, off + pos));
            pos += 8;
        }

        ImageOptionalHeader64 {
            magic: read_u16_le!(raw, off),
            major_linker_version: read_u8!(raw, off + 2),
            minor_linker_version: read_u8!(raw, off + 3),
            size_of_code: read_u32_le!(raw, off + 4),
            size_of_initialized_data: read_u32_le!(raw, off + 8),
            size_of_uninitialized_data: read_u32_le!(raw, off + 12),
            address_of_entry_point: read_u32_le!(raw, off + 16),
            base_of_code: read_u32_le!(raw, off + 20),
            image_base: read_u64_le!(raw, off + 24),
            section_alignment: read_u32_le!(raw, off + 32),
            file_alignment: read_u32_le!(raw, off + 36),
            major_operating_system_version: read_u16_le!(raw, off + 40),
            minor_operating_system_version: read_u16_le!(raw, off + 42),
            major_image_version: read_u16_le!(raw, off + 44),
            minor_image_version: read_u16_le!(raw, off + 46),
            major_subsystem_version: read_u16_le!(raw, off + 48),
            minor_subsystem_version: read_u16_le!(raw, off + 50),
            win32_version_value: read_u32_le!(raw, off + 52),
            size_of_image: read_u32_le!(raw, off + 56),
            size_of_headers: read_u32_le!(raw, off + 60),
            checksum: read_u32_le!(raw, off + 64),
            subsystem: read_u16_le!(raw, off + 68),
            dll_characteristics: read_u16_le!(raw, off + 70),
            size_of_stack_reserve: read_u64_le!(raw, off + 72),
            size_of_stack_commit: read_u64_le!(raw, off + 80),
            size_of_heap_reserve: read_u64_le!(raw, off + 88),
            size_of_heap_commit: read_u64_le!(raw, off + 94),
            loader_flags: read_u32_le!(raw, off + 102),
            number_of_rva_and_sizes: read_u32_le!(raw, off + 106),
            data_directory: dd,
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct TlsDirectory64 {
    pub tls_data_start: u64,
    pub tls_data_end: u64,
    pub tls_index: u64,
    pub tls_callbacks: u64,
    pub zero_fill_size: u32,
    pub characteristic: u32,
}

impl TlsDirectory64 {
    pub fn load(raw: &[u8], off: usize) -> TlsDirectory64 {
        TlsDirectory64 {
            tls_data_start: read_u64_le!(raw, off),
            tls_data_end: read_u64_le!(raw, off + 8),
            tls_index: read_u64_le!(raw, off + 16),
            tls_callbacks: read_u64_le!(raw, off + 24),
            zero_fill_size: read_u32_le!(raw, off + 32),
            characteristic: read_u32_le!(raw, off + 36),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
#[allow(dead_code)] // consumed by the import binding (mwemu-pe Phase 2)
pub struct DelayLoadIAT {
    pub(crate) name_ptr: u32,
    pub(crate) iat_addr: u64,
    pub(crate) bound_iat: u64,
}

#[allow(dead_code)]
impl DelayLoadIAT {
    pub(crate) fn load(raw: &[u8], off: usize) -> DelayLoadIAT {
        DelayLoadIAT {
            name_ptr: read_u32_le!(raw, off),
            iat_addr: read_u64_le!(raw, off + 4),
            bound_iat: read_u64_le!(raw, off + 8),
        }
    }
}

pub struct PE64 {
    pub filename: String,
    pub dos: ImageDosHeader,
    pub nt: ImageNtHeaders,
    pub fh: ImageFileHeader,
    pub opt: ImageOptionalHeader64,
    pub sect_hdr: Vec<ImageSectionHeader>,
    pub delay_load_dir: Vec<DelayLoadDirectory>,
    pub image_import_descriptor: Vec<ImageImportDescriptor>,
    /// Resolved import address -> function name. Filled during `load`/binding so
    /// `import_addr_to_name` works without keeping the file bytes around.
    pub iat_names: std::collections::HashMap<u64, String>,
}
