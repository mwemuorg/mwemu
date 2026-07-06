use super::{list_entry::ListEntry, unicode_string::UnicodeString};
use crate::maps::Maps;

#[derive(Debug)]
pub struct LdrDataTableEntry {
    pub in_load_order_links: ListEntry,           // +0x00 (8 bytes)
    pub in_memory_order_links: ListEntry,         // +8  (8 bytes)
    pub in_initialization_order_links: ListEntry, // +16 (8 bytes)
    pub dll_base: u32,                            // +24 +0x18 (4 bytes)
    pub entry_point: u32,                         // +28 +0x1C (4 bytes)
    pub size_of_image: u32,                       // +32 +0x20 (4 bytes)
    pub full_dll_name: UnicodeString,             // +36 +0x24 (8 bytes)
    pub base_dll_name: UnicodeString,             // +44 +0x2C (8 bytes)
    pub flags: u32,                               // +52 +0x34 (4 bytes)
    pub load_count: u16,                          // +56 +0x38 (2 bytes)
    pub tls_index: u16,                           // +58 +0x3A (2 bytes)
    pub hash_links: ListEntry,                    // +60 +0x3C (8 bytes)
    pub time_date_stamp: u32,                     // +68 +0x44 (4 bytes)
}

impl Default for LdrDataTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl LdrDataTableEntry {
    pub fn size() -> usize {
        72
    }

    pub fn new() -> LdrDataTableEntry {
        LdrDataTableEntry {
            in_load_order_links: ListEntry::new(),
            in_memory_order_links: ListEntry::new(),
            in_initialization_order_links: ListEntry::new(),
            dll_base: 0,
            entry_point: 0,
            size_of_image: 0,
            full_dll_name: UnicodeString::new(),
            base_dll_name: UnicodeString::new(),
            flags: 0,
            load_count: 0,
            tls_index: 0,
            hash_links: ListEntry::new(),
            time_date_stamp: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> LdrDataTableEntry {
        LdrDataTableEntry {
            in_load_order_links: ListEntry::load(addr, maps), // +0x00
            in_memory_order_links: ListEntry::load(addr + 8, maps), // +0x08
            in_initialization_order_links: ListEntry::load(addr + 16, maps), // +0x10
            dll_base: maps.read_dword(addr + 24).unwrap(),    // +0x18
            entry_point: maps.read_dword(addr + 28).unwrap(), // +0x1C
            size_of_image: maps.read_dword(addr + 32).unwrap(), // +0x20
            full_dll_name: UnicodeString::load(addr + 36, maps), // +0x24
            base_dll_name: UnicodeString::load(addr + 44, maps), // +0x2C
            flags: maps.read_dword(addr + 52).unwrap(),       // +0x34
            load_count: maps.read_word(addr + 56).unwrap(),   // +0x38
            tls_index: maps.read_word(addr + 58).unwrap(),    // +0x3A
            hash_links: ListEntry::load(addr + 60, maps),     // +0x3C
            time_date_stamp: maps.read_dword(addr + 68).unwrap(), // +0x44
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.in_load_order_links.save(addr, maps); // +0x00
        self.in_memory_order_links.save(addr + 8, maps); // +0x08
        self.in_initialization_order_links.save(addr + 16, maps); // +0x10
        maps.write_dword(addr + 24, self.dll_base); // +0x18
        maps.write_dword(addr + 28, self.entry_point); // +0x1C
        maps.write_dword(addr + 32, self.size_of_image); // +0x20
        self.full_dll_name.save(addr + 36, maps); // +0x24
        self.base_dll_name.save(addr + 44, maps); // +0x2C
        maps.write_dword(addr + 52, self.flags); // +0x34
        maps.write_word(addr + 56, self.load_count); // +0x38
        maps.write_word(addr + 58, self.tls_index); // +0x3A
        self.hash_links.save(addr + 60, maps); // +0x3C
        maps.write_dword(addr + 68, self.time_date_stamp); // +0x44
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
