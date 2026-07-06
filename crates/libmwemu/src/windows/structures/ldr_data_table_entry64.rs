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
    pub fn size() -> u64 {
        0x100
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
            in_memory_order_links: ListEntry64::load(addr + 0x10, maps),
            in_initialization_order_links: ListEntry64::load(addr + 0x20, maps),
            dll_base: maps.read_qword(addr + 0x30).unwrap(),
            entry_point: maps.read_qword(addr + 0x38).unwrap(),
            size_of_image: maps.read_dword(addr + 0x40).unwrap(), // dword aligned to qword
            full_dll_name: UnicodeString64::load(addr + 0x48, maps),
            base_dll_name: UnicodeString64::load(addr + 0x58, maps),
            flags: maps.read_dword(addr + 0x68).unwrap(),
            load_count: maps.read_word(addr + 0x6c).unwrap(),
            tls_index: maps.read_word(addr + 0x6e).unwrap(),
            hash_links: ListEntry64::load(addr + 0x70, maps),
            time_date_stamp: maps.read_dword(addr + 0x80).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.in_load_order_links.save(addr, maps);
        self.in_memory_order_links.save(addr + 0x10, maps);
        self.in_initialization_order_links.save(addr + 0x20, maps);
        maps.write_qword(addr + 0x30, self.dll_base);
        maps.write_qword(addr + 0x38, self.entry_point);
        maps.write_dword(addr + 0x40, self.size_of_image);
        self.full_dll_name.save(addr + 0x48, maps);
        self.base_dll_name.save(addr + 0x58, maps);
        maps.write_dword(addr + 0x68, self.flags);
        maps.write_word(addr + 0x6c, self.load_count);
        maps.write_word(addr + 0x6e, self.tls_index);
        self.hash_links.save(addr + 0x70, maps);
        maps.write_dword(addr + 0x80, self.time_date_stamp);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
