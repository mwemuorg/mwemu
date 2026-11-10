use crate::maps::Maps;

pub struct SystemInfo32 {
    oem_id: u32,
    processor_architecture: u32,
    reserved: u16,
    page_size: u32,
    min_app_addr: u32,
    max_app_addr: u32,
    active_processor_mask: u32,
    number_of_processors: u32,
    processor_type: u32,
    alloc_granularity: u32,
    processor_level: u16,
    processor_revision: u16,
}

impl Default for SystemInfo32 {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo32 {
    pub fn new() -> SystemInfo32 {
        SystemInfo32 {
            oem_id: 0x1337,
            processor_architecture: 9,
            reserved: 0,
            page_size: 4090,
            min_app_addr: 0,
            max_app_addr: 0,
            active_processor_mask: 1,
            number_of_processors: 4,
            processor_type: 586,
            alloc_granularity: 65536,
            processor_level: 5,
            processor_revision: 255,
        }
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.oem_id);
        maps.write_dword(addr + 4, self.processor_architecture);
        maps.write_word(addr + 8, self.reserved);
        maps.write_dword(addr + 10, self.page_size);
        maps.write_dword(addr + 14, self.min_app_addr);
        maps.write_dword(addr + 18, self.max_app_addr);
        maps.write_dword(addr + 22, self.active_processor_mask);
        maps.write_dword(addr + 26, self.number_of_processors);
        maps.write_dword(addr + 30, self.processor_type);
        maps.write_dword(addr + 34, self.alloc_granularity);
        maps.write_word(addr + 38, self.processor_level);
        maps.write_word(addr + 40, self.processor_revision);
    }

    pub fn size(&self) -> usize {
        42
    }
}
