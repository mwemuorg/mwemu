use crate::maps::Maps;

#[derive(Debug, Default)]
pub struct ListEntry64 {
    pub flink: u64,
    pub blink: u64,
}

impl ListEntry64 {
    pub(crate) const SIZE: u64 = 2 * size_of::<u64>() as u64;
    pub(crate) const BLINK_OFFSET: u64 = 0x08;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(addr: u64, maps: &Maps) -> ListEntry64 {
        ListEntry64 {
            flink: maps.read_qword(addr).unwrap_or(0),
            blink: maps.read_qword(addr + Self::BLINK_OFFSET).unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.flink);
        maps.write_qword(addr + Self::BLINK_OFFSET, self.blink);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
