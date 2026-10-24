use crate::maps::Maps;

/*
typedef struct _SCOPETABLE_ENTRY {
 DWORD EnclosingLevel;
 PVOID FilterFunc;
 PVOID HandlerFunc;
} SCOPETABLE_ENTRY, *PSCOPETABLE_ENTRY;
*/

#[derive(Debug)]
pub struct PScopeTableEntry {
    enclosing_level: u32,
    filter_func: u32,
    handler_func: u32,
}

impl PScopeTableEntry {
    pub fn load(addr: u64, maps: &Maps) -> PScopeTableEntry {
        PScopeTableEntry {
            enclosing_level: maps.read_dword(addr).unwrap_or(0),
            filter_func: maps.read_dword(addr + 4).unwrap_or(0),
            handler_func: maps.read_dword(addr + 8).unwrap_or(0),
        }
    }

    pub fn size() -> u64 {
        12
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
