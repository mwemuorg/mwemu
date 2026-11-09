use crate::emu;

#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct VS_FIXEDFILEINFO {
    pub dw_signature: u32,       // 0xFEEF04BD
    pub dw_struc_version: u32,   // 0x00010000
    pub dw_file_version_ms: u32, // 0x00060000a (6.10)
    pub dw_file_version_ls: u32, // 0x585d11bd (22621.4541)
    pub dw_product_version_ms: u32,
    pub dw_product_version_ls: u32,
    pub dw_file_flags_mask: u32,
    pub dw_file_flags: u32,
    pub dw_file_os: u32,
    pub dw_file_type: u32,
    pub dw_file_subtype: u32,
    pub dw_file_date_ms: u32,
    pub dw_file_date_ls: u32,
}

impl VS_FIXEDFILEINFO {
    pub fn size() -> usize {
        52 // 13 * 4 bytes
    }

    pub fn write(&self, emu: &mut emu::Emu, addr: u64) {
        emu.maps.write_dword(addr, self.dw_signature);
        emu.maps.write_dword(addr + 4, self.dw_struc_version);
        emu.maps.write_dword(addr + 8, self.dw_file_version_ms);
        emu.maps.write_dword(addr + 12, self.dw_file_version_ls);
        emu.maps.write_dword(addr + 16, self.dw_product_version_ms);
        emu.maps.write_dword(addr + 20, self.dw_product_version_ls);
        emu.maps.write_dword(addr + 24, self.dw_file_flags_mask);
        emu.maps.write_dword(addr + 28, self.dw_file_flags);
        emu.maps.write_dword(addr + 32, self.dw_file_os);
        emu.maps.write_dword(addr + 36, self.dw_file_type);
        emu.maps.write_dword(addr + 40, self.dw_file_subtype);
        emu.maps.write_dword(addr + 44, self.dw_file_date_ms);
        emu.maps.write_dword(addr + 48, self.dw_file_date_ls);
    }
}
