use crate::maps::Maps;

#[derive(Debug)]
pub struct OsVersionInfoExA {
    version_info_size: u32,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    platform_id: u32,
    sz_csd_version: [u8; 128],
    service_pack_major: u16,
    service_pack_minor: u16,
    suite_mask: u16,
    product_type: u8,
    reserved: u8,
}

impl Default for OsVersionInfoExA {
    fn default() -> Self {
        Self::new()
    }
}

impl OsVersionInfoExA {
    pub fn new() -> OsVersionInfoExA {
        let mut ovi = OsVersionInfoExA {
            version_info_size: 156, // size of OSVERSIONINFOEXA
            major_version: 10,
            minor_version: 0,
            build_number: 19042,
            platform_id: 2,
            sz_csd_version: [0; 128],
            service_pack_major: 0,
            service_pack_minor: 0,
            suite_mask: 0,
            product_type: 1, // VER_NT_WORKSTATION
            reserved: 0,
        };

        "Service Pack 0"
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &byte)| {
                ovi.sz_csd_version[i] = byte;
            });

        ovi
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.version_info_size);
        maps.write_dword(addr + 4, self.major_version);
        maps.write_dword(addr + 8, self.minor_version);
        maps.write_dword(addr + 12, self.build_number);
        maps.write_dword(addr + 16, self.platform_id);
        maps.write_buffer(addr + 20, &self.sz_csd_version);
        maps.write_word(addr + 148, self.service_pack_major);
        maps.write_word(addr + 150, self.service_pack_minor);
        maps.write_word(addr + 152, self.suite_mask);
        maps.write_byte(addr + 154, self.product_type);
        maps.write_byte(addr + 155, self.reserved);
    }
}
