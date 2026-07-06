use super::vs_fixedfileinfo::VS_FIXEDFILEINFO;
use crate::emu;

#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct VS_VERSIONINFO {
    pub w_length: u16,
    pub w_value_length: u16,
    pub w_type: u16,
    pub sz_key: [u16; 16], // "VS_VERSION_INFO\0" in UTF-16
    pub padding1: u16,
    pub value: VS_FIXEDFILEINFO,
    pub padding2: u16,
    pub children: u16,
}

impl Default for VS_VERSIONINFO {
    fn default() -> Self {
        Self::new()
    }
}

impl VS_VERSIONINFO {
    pub fn new() -> Self {
        let mut sz_key = [0u16; 16];
        // "VS_VERSION_INFO\0" in UTF-16
        let version_info = "VS_VERSION_INFO\0";
        for (i, ch) in version_info.chars().enumerate() {
            if i < 16 {
                sz_key[i] = ch as u16;
            }
        }

        VS_VERSIONINFO {
            w_length: 0,
            w_value_length: VS_FIXEDFILEINFO::size() as u16,
            w_type: 0,
            sz_key,
            padding1: 0,
            value: VS_FIXEDFILEINFO {
                dw_signature: 0xFEEF04BD,
                dw_struc_version: 0x00010000,
                dw_file_version_ms: 0x00060001,
                dw_file_version_ls: 0x00000000,
                dw_product_version_ms: 0x00060001,
                dw_product_version_ls: 0x00000000,
                dw_file_flags_mask: 0x3F,
                dw_file_flags: 0,
                dw_file_os: 0x40004, // VOS_NT_WINDOWS32
                dw_file_type: 0x2,   // VFT_DLL
                dw_file_subtype: 0,
                dw_file_date_ms: 0,
                dw_file_date_ls: 0,
            },
            padding2: 0,
            children: 0,
        }
    }

    pub fn size() -> usize {
        6 + 32 + 2 + VS_FIXEDFILEINFO::size() + 2 + 2
    }

    pub fn write(&self, emu: &mut emu::Emu, addr: u64) {
        let mut offset = addr;

        emu.maps.write_word(offset, self.w_length);
        offset += 2;

        emu.maps.write_word(offset, self.w_value_length);
        offset += 2;

        emu.maps.write_word(offset, self.w_type);
        offset += 2;

        // Write szKey
        for i in 0..16 {
            emu.maps.write_word(offset, self.sz_key[i]);
            offset += 2;
        }

        emu.maps.write_word(offset, self.padding1);
        offset += 2;

        self.value.write(emu, offset);
        offset += VS_FIXEDFILEINFO::size() as u64;

        emu.maps.write_word(offset, self.padding2);
        offset += 2;

        emu.maps.write_word(offset, self.children);
    }
}
