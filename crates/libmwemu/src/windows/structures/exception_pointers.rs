use crate::maps::Maps;

#[derive(Debug)]
pub struct ExceptionPointers {
    exception_record: u32,
    context_record: u32,
}

impl ExceptionPointers {
    pub fn load(addr: u64, maps: &Maps) -> ExceptionPointers {
        ExceptionPointers {
            exception_record: maps.read_dword(addr).unwrap(),
            context_record: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn size() -> u64 {
        8
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
