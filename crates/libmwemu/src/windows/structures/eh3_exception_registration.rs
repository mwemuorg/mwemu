use super::p_scope_table_entry::PScopeTableEntry;
use crate::maps::Maps;

#[derive(Debug)]
pub struct Eh3ExceptionRegistration {
    next: u32,
    exception_handler: u32,
    scope_table: PScopeTableEntry,
    try_level: u32,
}

impl Eh3ExceptionRegistration {
    pub fn load(addr: u64, maps: &Maps) -> Eh3ExceptionRegistration {
        Eh3ExceptionRegistration {
            next: maps.read_dword(addr).unwrap(),
            exception_handler: maps.read_dword(addr + 4).unwrap(),
            scope_table: PScopeTableEntry::load(addr + 8, maps),
            try_level: maps
                .read_dword(addr + 8 + PScopeTableEntry::size())
                .unwrap(),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
