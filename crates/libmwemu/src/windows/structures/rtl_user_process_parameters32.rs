use super::unicode_string::UnicodeString;
use crate::maps::Maps;

#[derive(Debug)]
pub struct RtlUserProcessParameters32 {
    pub reserved1: [u8; 16],
    pub reserved2: [u32; 10],
    pub image_path_name: UnicodeString,
    pub command_line: UnicodeString,
}

impl Default for RtlUserProcessParameters32 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtlUserProcessParameters32 {
    pub const ENVIRONMENT_OFFSET: u64 = 0x48;

    const RESERVED2_OFFSET: u64 = 0x10;
    const IMAGE_PATH_NAME_OFFSET: u64 = 0x38;
    const COMMAND_LINE_OFFSET: u64 = 0x40;

    pub fn new() -> Self {
        Self {
            reserved1: [0; 16],
            reserved2: [0; 10],
            image_path_name: UnicodeString::new(),
            command_line: UnicodeString::new(),
        }
    }

    pub fn size() -> usize {
        Self::ENVIRONMENT_OFFSET as usize + size_of::<u32>()
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.reserved1);
        for (i, val) in self.reserved2.iter().enumerate() {
            maps.write_dword(
                addr + Self::RESERVED2_OFFSET + (i * size_of::<u32>()) as u64,
                *val,
            );
        }
        self.image_path_name
            .save(addr + Self::IMAGE_PATH_NAME_OFFSET, maps);
        self.command_line
            .save(addr + Self::COMMAND_LINE_OFFSET, maps)
    }
}
