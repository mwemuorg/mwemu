use super::list_entry::ListEntry;
use crate::maps::Maps;

#[derive(Debug)]
pub struct PebLdrData {
    pub length: u32,
    pub initializated: u32,
    pub sshandle: u32,
    pub in_load_order_module_list: ListEntry, // 0x0c (12)
    pub in_memory_order_module_list: ListEntry,
    pub in_initialization_order_module_list: ListEntry,
    pub entry_in_progress: u32,
    pub shutdown_in_progress: u32,
    pub shutdown_thread_id: u32,
}

impl Default for PebLdrData {
    fn default() -> Self {
        Self::new()
    }
}

impl PebLdrData {
    pub fn size() -> usize {
        48
    }

    pub fn new() -> PebLdrData {
        PebLdrData {
            length: 48,
            initializated: 0,
            sshandle: 0,
            in_load_order_module_list: ListEntry::new(),
            in_memory_order_module_list: ListEntry::new(),
            in_initialization_order_module_list: ListEntry::new(),
            entry_in_progress: 0,
            shutdown_in_progress: 0,
            shutdown_thread_id: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PebLdrData {
        PebLdrData {
            length: maps.read_dword(addr).unwrap(),
            initializated: maps.read_dword(addr + 4).unwrap(),
            sshandle: maps.read_dword(addr + 8).unwrap(),
            in_load_order_module_list: ListEntry::load(addr + 12, maps),
            in_memory_order_module_list: ListEntry::load(addr + 20, maps),
            in_initialization_order_module_list: ListEntry::load(addr + 28, maps),
            entry_in_progress: maps.read_dword(addr + 36).unwrap(),
            shutdown_in_progress: maps.read_dword(addr + 40).unwrap(),
            shutdown_thread_id: maps.read_dword(addr + 44).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.length);
        maps.write_dword(addr + 4, self.initializated);
        maps.write_dword(addr + 8, self.sshandle);
        self.in_load_order_module_list.save(addr + 12, maps);
        self.in_memory_order_module_list.save(addr + 20, maps);
        self.in_initialization_order_module_list
            .save(addr + 28, maps);
        maps.write_dword(addr + 36, self.entry_in_progress);
        maps.write_dword(addr + 40, self.shutdown_in_progress);
        maps.write_dword(addr + 44, self.shutdown_thread_id);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
