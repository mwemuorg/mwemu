
use crate::pe::readers::{read_c_string, read_u64_le as read_u64_le_shared};
use super::{
    DelayLoadDirectory, ImageDosHeader, ImageExportDirectory, ImageFileHeader,
    ImageImportDescriptor, ImageNtHeaders, ImageOptionalHeader64, ImageSectionHeader, PE64,
    TlsDirectory64, IMAGE_DIRECTORY_ENTRY_DELAY_LOAD, IMAGE_DIRECTORY_ENTRY_EXPORT,
    IMAGE_DIRECTORY_ENTRY_IAT, IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_DIRECTORY_ENTRY_TLS,
    IMAGE_FILE_DLL, SECTION_HEADER_SZ,
};

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

impl PE64 {
    /// True if `raw` is a 64-bit PE image (x86-64 or ARM64), by its COFF
    /// `Machine` field.
    pub fn is_pe64(raw: &[u8]) -> bool {
        matches!(
            crate::pe::shared::pe_machine_type(raw),
            Some(crate::pe::shared::IMAGE_FILE_MACHINE_AMD64)
                | Some(crate::pe::shared::IMAGE_FILE_MACHINE_ARM64)
        )
    }

    /// Parse PE metadata from `raw`. Does **not** retain `raw`: the struct keeps
    /// only the parsed headers/sections/imports. The caller owns the bytes and
    /// passes them back to the accessors / `load` that need them.
    pub fn parse(filename: &str, raw: &[u8]) -> PE64 {
        let dos = ImageDosHeader::load(raw, 0);
        let nt = ImageNtHeaders::load(raw, dos.e_lfanew as usize);
        let fh = ImageFileHeader::load(raw, dos.e_lfanew as usize + 4);
        let opt = ImageOptionalHeader64::load(raw, dos.e_lfanew as usize + 24);
        let mut sect: Vec<ImageSectionHeader> = Vec::new();

        let mut off = dos.e_lfanew as usize + 24 + fh.size_of_optional_header as usize;
        for _ in 0..fh.number_of_sections {
            let s = ImageSectionHeader::load(raw, off);
            sect.push(s);
            off += SECTION_HEADER_SZ;
        }

        let import_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_IMPORT].virtual_address;
        let export_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_EXPORT].virtual_address;
        let delay_load_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_DELAY_LOAD].virtual_address;

        let mut image_import_descriptor: Vec<ImageImportDescriptor> = Vec::new();
        let mut delay_load_dir: Vec<DelayLoadDirectory> = Vec::new();

        if delay_load_va > 0 {
            let mut delay_load_off = PE64::vaddr_to_off(&sect, delay_load_va) as usize;
            if delay_load_off > 0 {
                loop {
                    let mut delay_load = DelayLoadDirectory::load(raw, delay_load_off);
                    if delay_load.handle == 0 || delay_load.name_ptr == 0 {
                        break;
                    }
                    let off = PE64::vaddr_to_off(&sect, delay_load.name_ptr) as usize;
                    if off > raw.len() {
                        panic!("the delay_load.name of pe64 is out of buffer");
                    }
                    delay_load.name = read_c_string(raw, off);
                    delay_load_dir.push(delay_load);
                    delay_load_off += DelayLoadDirectory::size();
                }
            }
        }

        if import_va > 0 {
            let mut import_off = PE64::vaddr_to_off(&sect, import_va) as usize;
            if import_off > 0 {
                loop {
                    let mut iid = ImageImportDescriptor::load(raw, import_off);
                    if iid.name_ptr == 0 {
                        break;
                    }
                    let off = PE64::vaddr_to_off(&sect, iid.name_ptr) as usize;
                    if off > raw.len() {
                        panic!("the name of pe64 iid is out of buffer");
                    }
                    iid.name = read_c_string(raw, off);
                    image_import_descriptor.push(iid);
                    import_off += ImageImportDescriptor::size();
                }
            }
        }

        let _exportd: Option<ImageExportDirectory> = if export_va > 0 { None } else { None };

        PE64 {
            filename: filename.to_string(),
            dos,
            fh,
            nt,
            opt,
            sect_hdr: sect,
            delay_load_dir,
            image_import_descriptor,
            iat_names: std::collections::HashMap::new(),
        }
    }

    pub fn mem_size(&self) -> usize {
        let mut sz = 0;
        for sect in &self.sect_hdr {
            if sect.virtual_size > sect.size_of_raw_data {
                sz += sect.virtual_size as usize;
            } else {
                sz += sect.size_of_raw_data as usize;
            }
        }
        sz
    }

    pub fn is_dll(&self) -> bool {
        self.fh.characteristics & IMAGE_FILE_DLL != 0
    }

    /// The PE headers slice (caller-owned bytes).
    pub fn headers<'a>(&self, raw: &'a [u8]) -> &'a [u8] {
        &raw[0..self.opt.size_of_headers as usize]
    }

    pub fn vaddr_to_off(sections: &Vec<ImageSectionHeader>, vaddr: u32) -> u32 {
        for sect in sections {
            if vaddr >= sect.virtual_address && vaddr < sect.virtual_address + sect.virtual_size {
                let offset_within_section = vaddr - sect.virtual_address;
                if offset_within_section >= sect.size_of_raw_data {
                    log::warn!(
                        "Virtual address 0x{:x} maps to uninitialized data in section '{}' (offset {} >= raw_size {})",
                        vaddr,
                        sect.get_name(),
                        offset_within_section,
                        sect.size_of_raw_data
                    );
                    return 0;
                }
                return sect.pointer_to_raw_data + offset_within_section;
            }
        }
        0
    }

    pub fn read_string(raw: &[u8], off: usize) -> String {
        read_c_string(raw, off)
    }

    pub fn num_of_sections(&self) -> usize {
        self.sect_hdr.len()
    }

    pub fn get_section_ptr_by_name<'a>(&self, raw: &'a [u8], name: &str) -> Option<&'a [u8]> {
        for sect in &self.sect_hdr {
            if sect.get_name() == name {
                let off = sect.pointer_to_raw_data as usize;
                let sz = sect.virtual_size as usize;
                return Some(&raw[off..off + sz]);
            }
        }
        None
    }

    pub fn get_section(&self, id: usize) -> &ImageSectionHeader {
        &self.sect_hdr[id]
    }

    pub fn get_pe_off(&self) -> u32 {
        self.dos.e_lfanew
    }

    pub fn get_section_ptr<'a>(&self, raw: &'a [u8], id: usize) -> &'a [u8] {
        if id > self.sect_hdr.len() {
            panic!("/!\\ warning: invalid section id {}", id);
        }
        let off = self.sect_hdr[id].pointer_to_raw_data as usize;
        let sz = self.sect_hdr[id].size_of_raw_data as usize;
        if off + sz > raw.len() {
            log::trace!(
                "/!\\ warning: id:{} name:{} raw sz:{} off:{} sz:{}  off+sz:{}",
                id,
                self.sect_hdr[id].get_name(),
                raw.len(),
                off,
                sz,
                off + sz
            );
            if off > raw.len() {
                return &[];
            }
            return &raw[off..];
        }
        &raw[off..off + sz]
    }

    pub fn get_section_vaddr(&self, id: usize) -> u32 {
        self.sect_hdr[id].virtual_address
    }

    pub fn get_tls_callbacks(&self, raw: &[u8], _vaddr: u32) -> Vec<u64> {
        let mut callbacks: Vec<u64> = Vec::new();

        if self.opt.data_directory.len() < IMAGE_DIRECTORY_ENTRY_TLS {
            log::trace!("/!\\ alert there is .tls section but not tls directory entry");
            return callbacks;
        }

        let entry_tls = self.opt.data_directory[IMAGE_DIRECTORY_ENTRY_TLS].virtual_address;
        let _iat = self.opt.data_directory[IMAGE_DIRECTORY_ENTRY_IAT].virtual_address;

        let tls_off = PE64::vaddr_to_off(&self.sect_hdr, entry_tls) as usize;
        let tls = TlsDirectory64::load(raw, tls_off);
        tls.print();

        let mut cb_off = PE64::vaddr_to_off(&self.sect_hdr, (tls.tls_callbacks & 0xffff) as u32);
        loop {
            let callback: u64 = read_u64_le!(raw, cb_off as usize);
            if callback == 0 {
                break;
            }
            log::trace!("0x{:x} TLS Callback: 0x{:x}", cb_off, callback);
            callbacks.push(callback);
            cb_off += 8;
        }

        callbacks
    }
}
