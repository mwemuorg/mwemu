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
    pub(crate) const INITIALIZED_OFFSET: u64 = 0x04;
    pub(crate) const IN_LOAD_ORDER_MODULE_LIST_OFFSET: u64 = 0x10;
    pub(crate) const IN_MEMORY_ORDER_MODULE_LIST_OFFSET: u64 = 0x20;
    pub(crate) const IN_INITIALIZATION_ORDER_MODULE_LIST_OFFSET: u64 = 0x30;

    const SSHANDLE_OFFSET: u64 = 0x08;
    const ENTRY_IN_PROGRESS_OFFSET: u64 = 0x40;

    pub fn size() -> usize {
        80
    }

    pub fn new() -> PebLdrData64 {
        PebLdrData64 {
            length: Self::size() as u32,
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
            length: maps.read_dword(addr).unwrap_or(0),
            initializated: maps
                .read_dword(addr + Self::INITIALIZED_OFFSET)
                .unwrap_or(0),
            sshandle: maps.read_qword(addr + Self::SSHANDLE_OFFSET).unwrap_or(0),
            in_load_order_module_list: ListEntry64::load(
                addr + Self::IN_LOAD_ORDER_MODULE_LIST_OFFSET,
                maps,
            ),
            in_memory_order_module_list: ListEntry64::load(
                addr + Self::IN_MEMORY_ORDER_MODULE_LIST_OFFSET,
                maps,
            ),
            in_initialization_order_module_list: ListEntry64::load(
                addr + Self::IN_INITIALIZATION_ORDER_MODULE_LIST_OFFSET,
                maps,
            ),
            entry_in_progress: ListEntry64::load(addr + Self::ENTRY_IN_PROGRESS_OFFSET, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.length);
        maps.write_dword(addr + Self::INITIALIZED_OFFSET, self.initializated);
        maps.write_qword(addr + Self::SSHANDLE_OFFSET, self.sshandle);
        self.in_load_order_module_list
            .save(addr + Self::IN_LOAD_ORDER_MODULE_LIST_OFFSET, maps);
        self.in_memory_order_module_list
            .save(addr + Self::IN_MEMORY_ORDER_MODULE_LIST_OFFSET, maps);
        self.in_initialization_order_module_list.save(
            addr + Self::IN_INITIALIZATION_ORDER_MODULE_LIST_OFFSET,
            maps,
        );
        self.entry_in_progress
            .save(addr + Self::ENTRY_IN_PROGRESS_OFFSET, maps);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
