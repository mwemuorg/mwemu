use crate::maps::Maps;

#[derive(Debug, Default)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: u32,
}

impl UnicodeString {
    const MAXIMUM_LENGTH_OFFSET: u64 = 0x02;
    const BUFFER_OFFSET: u64 = 0x04;

    pub fn size() -> u32 {
        8
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString {
        UnicodeString {
            length: maps.read_word(addr).unwrap_or(0),
            maximum_length: maps
                .read_word(addr + Self::MAXIMUM_LENGTH_OFFSET)
                .unwrap_or(0),
            buffer: maps.read_dword(addr + Self::BUFFER_OFFSET).unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + Self::MAXIMUM_LENGTH_OFFSET, self.maximum_length);
        maps.write_dword(addr + Self::BUFFER_OFFSET, self.buffer);
    }
}
