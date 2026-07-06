use crate::maps::Maps;

#[derive(Debug)]
pub struct CpInfo {
    pub max_char_size: u32,
    pub default_char: [u8; 2], // MAX_DEFAULTCHAR = 2
    pub lead_byte: [u8; 12],   // MAX_LEADBYTES = 12
}

impl Default for CpInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CpInfo {
    pub fn new() -> CpInfo {
        CpInfo {
            max_char_size: 1,
            default_char: [0x3F, 0], // '?' character as default
            lead_byte: [0; 12],
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> CpInfo {
        let mut info = CpInfo::new();
        info.max_char_size = maps.read_dword(addr).unwrap();

        // Read default char array
        for i in 0..2 {
            info.default_char[i] = maps.read_byte(addr + 4 + i as u64).unwrap();
        }

        // Read lead byte array
        for i in 0..12 {
            info.lead_byte[i] = maps.read_byte(addr + 6 + i as u64).unwrap();
        }

        info
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.max_char_size);

        // Write default char array
        for i in 0..2 {
            maps.write_byte(addr + 4 + i as u64, self.default_char[i]);
        }

        // Write lead byte array
        for i in 0..12 {
            maps.write_byte(addr + 6 + i as u64, self.lead_byte[i]);
        }
    }

    pub fn size() -> usize {
        18 // 4 bytes for max_char_size + 2 bytes for default_char + 12 bytes for lead_byte
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
