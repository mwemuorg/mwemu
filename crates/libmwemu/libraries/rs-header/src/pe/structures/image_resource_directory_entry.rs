#[derive(Debug)]
pub struct ImageResourceDirectoryEntry {
    pub name_or_id: u32,
    pub data_or_directory: u32,
}

impl ImageResourceDirectoryEntry {
    pub fn new() -> ImageResourceDirectoryEntry {
        ImageResourceDirectoryEntry {
            name_or_id: 0,
            data_or_directory: 0,
        }
    }

    pub fn size() -> usize {
        8
    }

    pub fn print(&self) {
        log::trace!(
            "name_or_id: {:x} data_or_directory: {:x}",
            self.name_or_id,
            self.data_or_directory
        );
    }

    pub fn is_name(&self) -> bool {
        self.name_or_id & 0x8000_0000 != 0
    }

    pub fn is_id(&self) -> bool {
        self.name_or_id & 0x8000_0000 == 0
    }

    pub fn get_name_or_id(&self) -> u32 {
        self.name_or_id & 0x7FFF_FFFF
    }

    pub fn is_directory(&self) -> bool {
        self.data_or_directory & 0x8000_0000 != 0
    }

    pub fn get_offset(&self) -> u32 {
        self.data_or_directory & 0x7FFF_FFFF
    }
}
