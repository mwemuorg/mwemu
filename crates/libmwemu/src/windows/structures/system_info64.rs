use crate::maps::Maps;

#[derive(Debug)]
pub struct SystemInfo64 {
    oem_id: u32,
    processor_architecture: u32,
    reserved: u16,
    page_size: u32,
    min_app_addr: u64,
    max_app_addr: u64,
    active_processor_mask: u64,
    number_of_processors: u32,
    processor_type: u32,
    alloc_granularity: u32,
    processor_level: u16,
    processor_revision: u16,
}

impl Default for SystemInfo64 {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo64 {
    pub fn new() -> SystemInfo64 {
        SystemInfo64 {
            oem_id: 0,
            processor_architecture: 9, // PROCESSOR_ARCHITECTURE_AMD64
            reserved: 0,
            page_size: 4096,
            min_app_addr: 0x10000,
            max_app_addr: 0x7FFFFFFEFFFF,
            active_processor_mask: 0xFF,
            number_of_processors: 8,
            processor_type: 8664,
            alloc_granularity: 65536,
            processor_level: 6,
            processor_revision: 0xA201,
        }
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        // First union/struct (4 bytes total)
        maps.write_word(addr, self.processor_architecture as u16);
        maps.write_word(addr + 2, self.reserved);

        // Rest of the structure
        maps.write_dword(addr + 4, self.page_size);
        maps.write_qword(addr + 8, self.min_app_addr);
        maps.write_qword(addr + 16, self.max_app_addr);
        maps.write_qword(addr + 24, self.active_processor_mask);
        maps.write_dword(addr + 32, self.number_of_processors);
        maps.write_dword(addr + 36, self.processor_type);
        maps.write_dword(addr + 40, self.alloc_granularity);
        maps.write_word(addr + 44, self.processor_level);
        maps.write_word(addr + 46, self.processor_revision);
    }

    pub fn size(&self) -> usize {
        48
    }
}
