use std::str;

use crate::pe::readers::{
    read_u16_le as read_u16_le_shared, read_u32_le as read_u32_le_shared,
};

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

pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
pub const IMAGE_FILE_MACHINE_ARM64: u16 = 0xAA64;

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
pub const IMAGE_OS2_SIGNATURE: u16 = 0x544E;
pub const IMAGE_OS2_SIGNATURE_LE: u16 = 0x45AC;
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550;
pub const IMAGE_SIZEOF_FILE_HEADER: u8 = 20;
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;
pub const SECTION_HEADER_SZ: usize = 40;

pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize = 3;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: usize = 6;
pub const IMAGE_DIRECTORY_ENTRY_COPYRIGHT: usize = 7;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize = 8;
pub const IMAGE_DIRECTORY_ENTRY_TLS: usize = 9;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize = 11;
pub const IMAGE_DIRECTORY_ENTRY_IAT: usize = 12;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_LOAD: usize = 13;
pub const IMAGE_DIRECTORY_ENTRY_DOTNET_HDR: usize = 14;

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;
pub const IMAGE_DEBUG_TYPE_UNKNOWN: u8 = 0;
pub const IMAGE_DEBUG_TYPE_COFF: u8 = 1;
pub const IMAGE_DEBUG_TYPE_CODEVIEW: u8 = 2;
pub const IMAGE_DEBUG_TYPE_FPO: u8 = 3;
pub const IMAGE_DEBUG_TYPE_MISC: u8 = 4;

#[derive(Debug)]
pub struct ImageDosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: u32,
}

impl ImageDosHeader {
    pub fn size() -> usize {
        64
    }

    pub fn load(raw: &[u8], off: usize) -> ImageDosHeader {
        ImageDosHeader {
            e_magic: read_u16_le!(raw, off),
            e_cblp: read_u16_le!(raw, off + 2),
            e_cp: read_u16_le!(raw, off + 4),
            e_crlc: read_u16_le!(raw, off + 6),
            e_cparhdr: read_u16_le!(raw, off + 8),
            e_minalloc: read_u16_le!(raw, off + 10),
            e_maxalloc: read_u16_le!(raw, off + 12),
            e_ss: read_u16_le!(raw, off + 14),
            e_sp: read_u16_le!(raw, off + 16),
            e_csum: read_u16_le!(raw, off + 18),
            e_ip: read_u16_le!(raw, off + 20),
            e_cs: read_u16_le!(raw, off + 22),
            e_lfarlc: read_u16_le!(raw, off + 24),
            e_ovno: read_u16_le!(raw, off + 26),
            e_res: [
                read_u16_le!(raw, off + 28),
                read_u16_le!(raw, off + 30),
                read_u16_le!(raw, off + 32),
                read_u16_le!(raw, off + 34),
            ],
            e_oemid: read_u16_le!(raw, off + 36),
            e_oeminfo: read_u16_le!(raw, off + 38),
            e_res2: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            e_lfanew: read_u32_le!(raw, off + 60),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageNtHeaders {
    pub signature: u32,
}

impl ImageNtHeaders {
    pub fn load(raw: &[u8], off: usize) -> ImageNtHeaders {
        ImageNtHeaders {
            signature: read_u32_le!(raw, off),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

impl ImageFileHeader {
    pub fn load(raw: &[u8], off: usize) -> ImageFileHeader {
        ImageFileHeader {
            machine: read_u16_le!(raw, off),
            number_of_sections: read_u16_le!(raw, off + 2),
            time_date_stamp: read_u32_le!(raw, off + 4),
            pointer_to_symbol_table: read_u32_le!(raw, off + 8),
            number_of_symbols: read_u32_le!(raw, off + 12),
            size_of_optional_header: read_u16_le!(raw, off + 16),
            characteristics: read_u16_le!(raw, off + 18),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

impl ImageDataDirectory {
    pub fn load(raw: &[u8], off: usize) -> ImageDataDirectory {
        ImageDataDirectory {
            virtual_address: read_u32_le!(raw, off),
            size: read_u32_le!(raw, off + 4),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageSectionHeader {
    pub name: [u8; IMAGE_SIZEOF_SHORT_NAME],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

impl ImageSectionHeader {
    pub fn load(raw: &[u8], off: usize) -> ImageSectionHeader {
        let mut name: [u8; IMAGE_SIZEOF_SHORT_NAME] = [0; IMAGE_SIZEOF_SHORT_NAME];
        name[..(off + IMAGE_SIZEOF_SHORT_NAME - off)]
            .copy_from_slice(&raw[off..(off + IMAGE_SIZEOF_SHORT_NAME)]);

        let off2 = off + IMAGE_SIZEOF_SHORT_NAME;

        ImageSectionHeader {
            name,
            virtual_size: read_u32_le!(raw, off2),
            virtual_address: read_u32_le!(raw, off2 + 4),
            size_of_raw_data: read_u32_le!(raw, off2 + 8),
            pointer_to_raw_data: read_u32_le!(raw, off2 + 12),
            pointer_to_relocations: read_u32_le!(raw, off2 + 16),
            pointer_to_linenumbers: read_u32_le!(raw, off2 + 20),
            number_of_relocations: read_u16_le!(raw, off2 + 24),
            number_of_linenumbers: read_u16_le!(raw, off2 + 26),
            characteristics: read_u32_le!(raw, off2 + 28),
        }
    }

    pub fn get_name(&self) -> String {
        let s = str::from_utf8(&self.name).unwrap_or("err");
        s.to_string().replace("\x00", "")
    }

    pub fn set_name(&mut self, newname: &str) {
        if newname.len() + 1 > IMAGE_SIZEOF_SHORT_NAME {
            panic!("fixing a name bigger than IMAGE_SIZEOF_SHORT_NAME");
        }
        let mut vname: Vec<u8> = newname.as_bytes().to_vec();
        vname.push(0);
        for (i, &item) in vname.iter().enumerate() {
            self.name[i] = item;
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageResourceDirectoryEntry {
    pub name: u32,
    pub offset_to_data: u32,
}

impl ImageResourceDirectoryEntry {
    pub fn load(raw: &[u8], off: usize) -> ImageResourceDirectoryEntry {
        ImageResourceDirectoryEntry {
            name: read_u32_le!(raw, off),
            offset_to_data: read_u32_le!(raw, off + 4),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageResourceDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub number_of_named_entries: u16,
    pub number_of_id_entries: u16,
}

impl ImageResourceDirectory {
    pub fn load(raw: &[u8], off: usize) -> ImageResourceDirectory {
        ImageResourceDirectory {
            characteristics: read_u32_le!(raw, off),
            time_date_stamp: read_u32_le!(raw, off + 4),
            major_version: read_u16_le!(raw, off + 8),
            minor_version: read_u16_le!(raw, off + 10),
            number_of_named_entries: read_u16_le!(raw, off + 12),
            number_of_id_entries: read_u16_le!(raw, off + 14),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageExportDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub name: u32,
    pub base: u32,
    pub number_of_functions: u32,
    pub number_of_names: u32,
    pub address_of_functions: u32,
    pub address_of_names: u32,
    pub address_of_name_ordinals: u32,
}

impl ImageExportDirectory {
    pub fn load(raw: &[u8], off: usize) -> ImageExportDirectory {
        ImageExportDirectory {
            characteristics: read_u32_le!(raw, off),
            time_date_stamp: read_u32_le!(raw, off + 4),
            major_version: read_u16_le!(raw, off + 8),
            minor_version: read_u16_le!(raw, off + 10),
            name: read_u32_le!(raw, off + 12),
            base: read_u32_le!(raw, off + 16),
            number_of_functions: read_u32_le!(raw, off + 20),
            number_of_names: read_u32_le!(raw, off + 24),
            address_of_functions: read_u32_le!(raw, off + 28),
            address_of_names: read_u32_le!(raw, off + 32),
            address_of_name_ordinals: read_u32_le!(raw, off + 36),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct DelayLoadDirectory {
    pub attributes: u32,
    pub name_ptr: u32,
    pub handle: u32,
    pub address_table: u32,
    pub name_table: u32,
    pub bound_delay_import_table: u32,
    pub unload_delay_import_table: u32,
    pub tstamp: u32,
    pub name: String,
}

impl DelayLoadDirectory {
    pub fn size() -> usize {
        32
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }

    pub fn load(raw: &[u8], off: usize) -> DelayLoadDirectory {
        DelayLoadDirectory {
            attributes: read_u32_le!(raw, off),
            name_ptr: read_u32_le!(raw, off + 4),
            handle: read_u32_le!(raw, off + 8),
            address_table: read_u32_le!(raw, off + 12),
            name_table: read_u32_le!(raw, off + 16),
            bound_delay_import_table: read_u32_le!(raw, off + 20),
            unload_delay_import_table: read_u32_le!(raw, off + 24),
            tstamp: read_u32_le!(raw, off + 28),
            name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct ImageImportDirectory {
    pub address_of_import_lookup_table: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub address_of_names: u32,
    pub address_of_import_table: u32,
}

impl ImageImportDirectory {
    pub fn load(raw: &[u8], off: usize) -> ImageImportDirectory {
        ImageImportDirectory {
            address_of_import_lookup_table: read_u32_le!(raw, off),
            time_date_stamp: read_u32_le!(raw, off + 4),
            forwarder_chain: read_u32_le!(raw, off + 8),
            address_of_names: read_u32_le!(raw, off + 12),
            address_of_import_table: read_u32_le!(raw, off + 16),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageImportDescriptor {
    pub original_first_thunk: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub name_ptr: u32,
    pub first_thunk: u32,
    pub name: String,
}

impl ImageImportDescriptor {
    pub fn load(raw: &[u8], off: usize) -> ImageImportDescriptor {
        ImageImportDescriptor {
            original_first_thunk: read_u32_le!(raw, off),
            time_date_stamp: read_u32_le!(raw, off + 4),
            forwarder_chain: read_u32_le!(raw, off + 8),
            name_ptr: read_u32_le!(raw, off + 12),
            first_thunk: read_u32_le!(raw, off + 16),
            name: String::new(),
        }
    }

    pub fn size() -> usize {
        20
    }
}

#[derive(Debug)]
pub struct ImportLookupTable {
    pub bits: Vec<u32>,
}

#[derive(Debug)]
pub struct HintNameItem {
    pub is_ordinal: bool,
    pub func_name_addr: u32,
}

impl HintNameItem {
    pub fn load(raw: &[u8], off: usize) -> HintNameItem {
        if raw.len() <= off + 4 {
            HintNameItem {
                is_ordinal: false,
                func_name_addr: 0,
            }
        } else {
            HintNameItem {
                is_ordinal: raw[off] & 0b10000000 == 0b10000000,
                func_name_addr: read_u32_le!(raw, off),
            }
        }
    }

    pub fn size() -> usize {
        4
    }
}

#[derive(Debug)]
pub struct ImportAddressTable {}

impl ImportLookupTable {
    pub fn load(_raw: &[u8], _off: usize, _nitems: usize) -> ImportLookupTable {
        ImportLookupTable { bits: Vec::new() }
    }
}

#[derive(Debug)]
pub struct TagImportDirectory {
    pub dw_rva_function_name_list: u32,
    pub dw_useless1: u32,
    pub dw_useless2: u32,
    pub dw_rva_module_name: u32,
    pub dw_rva_function_address_list: u32,
}

impl TagImportDirectory {
    pub fn load(raw: &[u8], off: usize) -> TagImportDirectory {
        TagImportDirectory {
            dw_rva_function_name_list: read_u32_le!(raw, off),
            dw_useless1: read_u32_le!(raw, off + 4),
            dw_useless2: read_u32_le!(raw, off + 8),
            dw_rva_module_name: read_u32_le!(raw, off + 12),
            dw_rva_function_address_list: read_u32_le!(raw, off + 16),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageDebugDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub types: u32,
    pub size_of_data: u32,
    pub address_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
}

impl ImageDebugDirectory {
    pub fn load(raw: &[u8], off: usize) -> ImageDebugDirectory {
        ImageDebugDirectory {
            characteristics: read_u32_le!(raw, off),
            time_date_stamp: read_u32_le!(raw, off + 4),
            major_version: read_u16_le!(raw, off + 8),
            minor_version: read_u16_le!(raw, off + 10),
            types: read_u32_le!(raw, off + 12),
            size_of_data: read_u32_le!(raw, off + 16),
            address_of_raw_data: read_u32_le!(raw, off + 20),
            pointer_to_raw_data: read_u32_le!(raw, off + 24),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

#[derive(Debug)]
pub struct ImageBaseRelocation {
    pub virtual_address: u32,
    pub size_of_block: u32,
}

impl ImageBaseRelocation {
    pub fn load(raw: &[u8], off: usize) -> ImageBaseRelocation {
        ImageBaseRelocation {
            virtual_address: read_u32_le!(raw, off),
            size_of_block: read_u32_le!(raw, off + 4),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}

pub struct Section {
    pub name: String,
    pub off: usize,
    pub sz: usize,
}

impl Section {
    pub fn new(off: usize, sz: usize) -> Section {
        Section {
            name: String::new(),
            off,
            sz,
        }
    }
}

/// Read the COFF `Machine` field (architecture) from a PE image's bytes, or
/// `None` if `raw` is not a PE. The value is one of the `IMAGE_FILE_MACHINE_*`
/// constants. Byte-based, mirroring ELF's `Elf64::elf_machine`.
pub fn pe_machine_type(raw: &[u8]) -> Option<u16> {
    if raw.len() < ImageDosHeader::size() {
        return None;
    }
    let dos = ImageDosHeader::load(raw, 0);
    if dos.e_magic != IMAGE_DOS_SIGNATURE {
        return None;
    }

    // e_lfanew points to the PE signature (4 bytes) followed by the COFF file
    // header; the Machine field is its first 2 bytes (at e_lfanew + 4).
    let nt_off = dos.e_lfanew as usize;
    if nt_off + 6 > raw.len() {
        return None;
    }
    if read_u32_le_shared(raw, nt_off) != IMAGE_NT_SIGNATURE {
        return None;
    }
    Some(read_u16_le_shared(raw, nt_off + 4))
}
