use crate::maps::Maps;

#[derive(Debug, Default)]
pub struct UnicodeString64 {
    pub length: u16,
    pub maximum_length: u16,
    pub padding: u32,
    pub buffer: u64,
}

impl UnicodeString64 {
    pub(crate) const MAXIMUM_LENGTH_OFFSET: u64 = 0x02;
    pub(crate) const BUFFER_OFFSET: u64 = 0x08;

    const PADDING_OFFSET: u64 = 0x04;

    pub fn size() -> u64 {
        16
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString64 {
        UnicodeString64 {
            length: maps.read_word(addr).unwrap_or(0),
            maximum_length: maps
                .read_word(addr + Self::MAXIMUM_LENGTH_OFFSET)
                .unwrap_or(0),
            padding: maps.read_dword(addr + Self::PADDING_OFFSET).unwrap_or(0),
            buffer: maps.read_qword(addr + Self::BUFFER_OFFSET).unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + Self::MAXIMUM_LENGTH_OFFSET, self.maximum_length);
        maps.write_dword(addr + Self::PADDING_OFFSET, self.padding);
        maps.write_qword(addr + Self::BUFFER_OFFSET, self.buffer);
    }
}
