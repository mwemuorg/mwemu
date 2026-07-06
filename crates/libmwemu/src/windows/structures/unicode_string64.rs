use crate::maps::Maps;

#[derive(Debug)]
pub struct UnicodeString64 {
    pub length: u16,         // 0x58          0x68
    pub maximum_length: u16, // 0x5a  0x6a
    pub padding: u32,        // 0x5c         0x6c
    pub buffer: u64,         // 0x60         0x70
}

impl Default for UnicodeString64 {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeString64 {
    pub fn size() -> u64 {
        16
    }

    pub fn new() -> UnicodeString64 {
        UnicodeString64 {
            length: 0,
            maximum_length: 0,
            padding: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString64 {
        UnicodeString64 {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            padding: maps.read_dword(addr + 4).unwrap(),
            buffer: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_dword(addr + 4, self.padding);
        maps.write_qword(addr + 8, self.buffer);
    }
}
