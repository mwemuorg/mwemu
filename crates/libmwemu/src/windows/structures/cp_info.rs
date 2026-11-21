use crate::maps::Maps;

#[repr(C)]
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
        info.max_char_size = maps.read_dword(addr).unwrap_or(0);

        // Read default char array
        for i in 0..2 {
            info.default_char[i] = maps.read_byte(addr + 4 + i as u64).unwrap_or(0);
        }

        // Read lead byte array
        for i in 0..12 {
            info.lead_byte[i] = maps.read_byte(addr + 6 + i as u64).unwrap_or(0);
        }

        info
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.save_fields(addr, maps);
    }

    pub(crate) fn save_fields(&self, addr: u64, maps: &mut Maps) -> bool {
        // GetCPInfo writes the 18 data bytes. The trailing alignment bytes are unspecified.
        let mut bytes = [0; 18];
        bytes[..4].copy_from_slice(&self.max_char_size.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.default_char);
        bytes[6..].copy_from_slice(&self.lead_byte);
        maps.validate_write_range(addr, bytes.len() as u64) && maps.write_bytes(addr, &bytes)
    }

    pub fn size() -> usize {
        size_of::<Self>()
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
