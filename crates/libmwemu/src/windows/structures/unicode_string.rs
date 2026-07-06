use crate::maps::Maps;

#[derive(Debug)]
pub struct UnicodeString {
    pub length: u16,         // 0x58          0x68
    pub maximum_length: u16, // 0x5a  0x6a
    pub buffer: u32,         // 0x60         0x70
}

impl Default for UnicodeString {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeString {
    pub fn size() -> u32 {
        8
    }

    pub fn new() -> UnicodeString {
        UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString {
        UnicodeString {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            buffer: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_dword(addr + 4, self.buffer);
    }
}
