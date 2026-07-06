use crate::maps::Maps;

#[derive(Debug)]
pub struct ListEntry {
    pub flink: u32,
    pub blink: u32,
}

impl Default for ListEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl ListEntry {
    pub fn new() -> ListEntry {
        ListEntry { flink: 0, blink: 0 }
    }

    pub fn load(addr: u64, maps: &Maps) -> ListEntry {
        ListEntry {
            flink: maps.read_dword(addr).unwrap(),
            blink: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.flink);
        maps.write_dword(addr + 4, self.blink);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
