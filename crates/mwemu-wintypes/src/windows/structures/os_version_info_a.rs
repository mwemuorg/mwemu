use crate::maps::Maps;

#[derive(Debug)]
pub struct OsVersionInfoA {
    version_info_size: u32,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    platform_id: u32,
    sz_csd_version: [u8; 128],
}

impl Default for OsVersionInfoA {
    fn default() -> Self {
        Self::new()
    }
}

impl OsVersionInfoA {
    pub fn new() -> OsVersionInfoA {
        let mut ovi = OsVersionInfoA {
            version_info_size: 148, // size of OSVERSIONINFOA
            major_version: 10,
            minor_version: 0,
            build_number: 19042,
            platform_id: 2,
            sz_csd_version: [0; 128],
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
    }
}
