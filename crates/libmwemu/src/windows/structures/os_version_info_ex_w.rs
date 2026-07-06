use crate::maps::Maps;

#[derive(Debug)]
pub struct OsVersionInfoExW {
    version_info_size: u32,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    platform_id: u32,
    version: [u16; 128],     // WCHAR array (UTF-16)
    service_pack_major: u16, // WORD
    service_pack_minor: u16, // WORD
    suite_mask: u16,         // WORD
    product_type: u8,        // BYTE
    reserved: u8,            // BYTE
}

impl Default for OsVersionInfoExW {
    fn default() -> Self {
        Self::new()
    }
}

impl OsVersionInfoExW {
    pub fn new() -> OsVersionInfoExW {
        let mut ovi = OsVersionInfoExW {
            version_info_size: 284,
            major_version: 10,
            minor_version: 0,
            build_number: 19042,
            platform_id: 2,
            version: [0; 128],
            service_pack_major: 0,
            service_pack_minor: 0,
            suite_mask: 0,
            product_type: 1, // VER_NT_WORKSTATION
            reserved: 0,
        };

        // Convert "Service Pack 0" to UTF-16
        let service_pack_str = "Service Pack 0";
        let utf16_chars: Vec<u16> = service_pack_str.encode_utf16().collect();

        for (i, &char_code) in utf16_chars.iter().enumerate() {
            if i < 127 {
                // Leave room for null terminator
                ovi.version[i] = char_code;
            }
        }
        // Null terminator is already set (array initialized with zeros)

        ovi
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.version_info_size);
        maps.write_dword(addr + 4, self.major_version);
        maps.write_dword(addr + 8, self.minor_version);
        maps.write_dword(addr + 12, self.build_number);
        maps.write_dword(addr + 16, self.platform_id);

        // Write WCHAR array (each element is 2 bytes)
        for (i, &char_code) in self.version.iter().enumerate() {
            maps.write_word(addr + 20 + (i as u64 * 2), char_code);
        }

        maps.write_word(addr + 276, self.service_pack_major);
        maps.write_word(addr + 278, self.service_pack_minor);
        maps.write_word(addr + 280, self.suite_mask);
        maps.write_byte(addr + 282, self.product_type);
        maps.write_byte(addr + 283, self.reserved);
    }
}
