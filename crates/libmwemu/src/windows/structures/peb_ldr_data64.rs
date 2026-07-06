use super::list_entry64::ListEntry64;
use crate::maps::Maps;

#[derive(Debug)]
pub struct PebLdrData64 {
    pub length: u32,
    pub initializated: u32,
    pub sshandle: u64,
    pub in_load_order_module_list: ListEntry64,
    pub in_memory_order_module_list: ListEntry64,
    pub in_initialization_order_module_list: ListEntry64,
    pub entry_in_progress: ListEntry64,
}

impl Default for PebLdrData64 {
    fn default() -> Self {
        Self::new()
    }
}

impl PebLdrData64 {
    pub fn size() -> usize {
        80
    }

    pub fn new() -> PebLdrData64 {
        PebLdrData64 {
            length: 80, // prev:72
            initializated: 0,
            sshandle: 0,
            in_load_order_module_list: ListEntry64::new(),
            in_memory_order_module_list: ListEntry64::new(),
            in_initialization_order_module_list: ListEntry64::new(),
            entry_in_progress: ListEntry64::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PebLdrData64 {
        PebLdrData64 {
            length: maps.read_dword(addr).unwrap(),
            initializated: maps.read_dword(addr + 4).unwrap(),
            sshandle: maps.read_qword(addr + 8).unwrap(),
            in_load_order_module_list: ListEntry64::load(addr + 0x10, maps),
            in_memory_order_module_list: ListEntry64::load(addr + 0x20, maps),
            in_initialization_order_module_list: ListEntry64::load(addr + 0x30, maps),
            entry_in_progress: ListEntry64::load(addr + 0x40, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.length);
        maps.write_dword(addr + 4, self.initializated);
        maps.write_qword(addr + 8, self.sshandle);
        self.in_load_order_module_list.save(addr + 0x10, maps);
        self.in_memory_order_module_list.save(addr + 0x20, maps);
        self.in_initialization_order_module_list
            .save(addr + 0x30, maps);
        self.entry_in_progress.save(addr + 0x40, maps);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
