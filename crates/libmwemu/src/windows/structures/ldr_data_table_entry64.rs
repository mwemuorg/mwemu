use super::{list_entry64::ListEntry64, unicode_string64::UnicodeString64};
use crate::maps::Maps;

#[derive(Debug)]
pub struct LdrDataTableEntry64 {
    pub in_load_order_links: ListEntry64,
    pub in_memory_order_links: ListEntry64,
    pub in_initialization_order_links: ListEntry64,
    pub dll_base: u64,
    pub entry_point: u64,
    pub size_of_image: u32,
    pub full_dll_name: UnicodeString64,
    pub base_dll_name: UnicodeString64,
    pub flags: u32,
    pub load_count: u16,
    pub tls_index: u16,
    pub hash_links: ListEntry64,
    pub time_date_stamp: u32,
}

impl Default for LdrDataTableEntry64 {
    fn default() -> Self {
        Self::new()
    }
}

impl LdrDataTableEntry64 {
    pub(crate) const BOOTSTRAP_LOAD_REASON: u32 = 9;

    pub(crate) const IN_MEMORY_ORDER_LINKS_OFFSET: u64 = 0x10;
    pub(crate) const IN_INITIALIZATION_ORDER_LINKS_OFFSET: u64 = 0x20;
    pub(crate) const DLL_BASE_OFFSET: u64 = 0x30;
    pub(crate) const FULL_DLL_NAME_OFFSET: u64 = 0x48;
    pub(crate) const BASE_DLL_NAME_OFFSET: u64 = 0x58;
    pub(crate) const HASH_LINKS_OFFSET: u64 = 0x70;
    pub(crate) const DDAG_NODE_OFFSET: u64 = 0x98;
    pub(crate) const BASE_ADDRESS_INDEX_NODE_OFFSET: u64 = 0xc8;
    pub(crate) const LOAD_REASON_OFFSET: u64 = 0x10c;

    const ENTRY_POINT_OFFSET: u64 = 0x38;
    const SIZE_OF_IMAGE_OFFSET: u64 = 0x40;
    const FLAGS_OFFSET: u64 = 0x68;
    const LOAD_COUNT_OFFSET: u64 = 0x6c;
    const TLS_INDEX_OFFSET: u64 = 0x6e;
    const TIME_DATE_STAMP_OFFSET: u64 = 0x80;

    pub fn size() -> u64 {
        0x120
    }

    pub fn new() -> LdrDataTableEntry64 {
        LdrDataTableEntry64 {
            in_load_order_links: ListEntry64::new(),
            in_memory_order_links: ListEntry64::new(),
            in_initialization_order_links: ListEntry64::new(),
            dll_base: 0,
            entry_point: 0,
            size_of_image: 0,
            full_dll_name: UnicodeString64::new(),
            base_dll_name: UnicodeString64::new(),
            flags: 0,
            load_count: 0,
            tls_index: 0,
            hash_links: ListEntry64::new(),
            time_date_stamp: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> LdrDataTableEntry64 {
        LdrDataTableEntry64 {
            in_load_order_links: ListEntry64::load(addr, maps),
            in_memory_order_links: ListEntry64::load(
                addr + Self::IN_MEMORY_ORDER_LINKS_OFFSET,
                maps,
            ),
            in_initialization_order_links: ListEntry64::load(
                addr + Self::IN_INITIALIZATION_ORDER_LINKS_OFFSET,
                maps,
            ),
            dll_base: maps.read_qword(addr + Self::DLL_BASE_OFFSET).unwrap_or(0),
            entry_point: maps
                .read_qword(addr + Self::ENTRY_POINT_OFFSET)
                .unwrap_or(0),
            size_of_image: maps
                .read_dword(addr + Self::SIZE_OF_IMAGE_OFFSET)
                .unwrap_or(0),
            full_dll_name: UnicodeString64::load(addr + Self::FULL_DLL_NAME_OFFSET, maps),
            base_dll_name: UnicodeString64::load(addr + Self::BASE_DLL_NAME_OFFSET, maps),
            flags: maps.read_dword(addr + Self::FLAGS_OFFSET).unwrap_or(0),
            load_count: maps.read_word(addr + Self::LOAD_COUNT_OFFSET).unwrap_or(0),
            tls_index: maps.read_word(addr + Self::TLS_INDEX_OFFSET).unwrap_or(0),
            hash_links: ListEntry64::load(addr + Self::HASH_LINKS_OFFSET, maps),
            time_date_stamp: maps
                .read_dword(addr + Self::TIME_DATE_STAMP_OFFSET)
                .unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.in_load_order_links.save(addr, maps);
        self.in_memory_order_links
            .save(addr + Self::IN_MEMORY_ORDER_LINKS_OFFSET, maps);
        self.in_initialization_order_links
            .save(addr + Self::IN_INITIALIZATION_ORDER_LINKS_OFFSET, maps);
        maps.write_qword(addr + Self::DLL_BASE_OFFSET, self.dll_base);
        maps.write_qword(addr + Self::ENTRY_POINT_OFFSET, self.entry_point);
        maps.write_dword(addr + Self::SIZE_OF_IMAGE_OFFSET, self.size_of_image);
        self.full_dll_name
            .save(addr + Self::FULL_DLL_NAME_OFFSET, maps);
        self.base_dll_name
            .save(addr + Self::BASE_DLL_NAME_OFFSET, maps);
        maps.write_dword(addr + Self::FLAGS_OFFSET, self.flags);
        maps.write_word(addr + Self::LOAD_COUNT_OFFSET, self.load_count);
        maps.write_word(addr + Self::TLS_INDEX_OFFSET, self.tls_index);
        self.hash_links.save(addr + Self::HASH_LINKS_OFFSET, maps);
        maps.write_dword(addr + Self::TIME_DATE_STAMP_OFFSET, self.time_date_stamp);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
