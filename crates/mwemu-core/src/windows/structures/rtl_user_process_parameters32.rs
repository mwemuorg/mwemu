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
    pub fn new() -> Self {
        Self {
            reserved1: [0; 16],
            reserved2: [0; 10],
            image_path_name: UnicodeString::new(),
            command_line: UnicodeString::new(),
        }
    }

    pub fn size() -> usize {
        72
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.reserved1);
        for (i, val) in self.reserved2.iter().enumerate() {
            maps.write_dword(addr + 16 + (i * 4) as u64, *val);
        }
        self.image_path_name.save(addr + 16 + 40, maps);
        self.command_line.save(addr + 16 + 40 + 8, maps)
    }
}
