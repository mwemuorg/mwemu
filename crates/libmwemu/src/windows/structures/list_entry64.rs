use crate::maps::Maps;

#[derive(Debug)]
pub struct ListEntry64 {
    pub flink: u64,
    pub blink: u64,
}

impl Default for ListEntry64 {
    fn default() -> Self {
        Self::new()
    }
}

impl ListEntry64 {
    pub fn new() -> ListEntry64 {
        ListEntry64 { flink: 0, blink: 0 }
    }

    pub fn load(addr: u64, maps: &Maps) -> ListEntry64 {
        ListEntry64 {
            flink: maps.read_qword(addr).unwrap(),
            blink: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.flink);
        maps.write_qword(addr + 8, self.blink);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
