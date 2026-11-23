use crate::maps::Maps;

#[derive(Debug, Default)]
pub struct ListEntry {
    pub flink: u32,
    pub blink: u32,
}

impl ListEntry {
    pub(crate) const BLINK_OFFSET: u64 = 0x04;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(addr: u64, maps: &Maps) -> ListEntry {
        ListEntry {
            flink: maps.read_dword(addr).unwrap_or(0),
            blink: maps.read_dword(addr + Self::BLINK_OFFSET).unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.flink);
        maps.write_dword(addr + Self::BLINK_OFFSET, self.blink);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
