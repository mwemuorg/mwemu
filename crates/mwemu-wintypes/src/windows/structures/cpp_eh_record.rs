use super::p_scope_table_entry::PScopeTableEntry;
use crate::maps::Maps;

#[derive(Debug)]
pub struct CppEhRecord {
    old_esp: u32,
    exc_ptr: u32,
    next: u32, // ptr to _EH3_EXCEPTION_REGISTRATION
    exception_handler: u32,
    scope_table: PScopeTableEntry,
    try_level: u32,
}

impl CppEhRecord {
    pub fn load(addr: u64, maps: &Maps) -> CppEhRecord {
        CppEhRecord {
            old_esp: maps.read_dword(addr).unwrap_or(0),
            exc_ptr: maps.read_dword(addr + 4).unwrap_or(0),
            next: maps.read_dword(addr + 8).unwrap_or(0),
            exception_handler: maps.read_dword(addr + 12).unwrap_or(0),
            scope_table: PScopeTableEntry::load(addr + 16, maps),
            try_level: maps
                .read_dword(addr + 16 + PScopeTableEntry::size())
                .unwrap_or(0),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
