
use crate::pe::readers::{
    read_c_string, read_c_string_with_max, read_u32_le as read_u32_le_shared,
};
use super::{
    DelayLoadDirectory, ImageDosHeader, ImageExportDirectory, ImageFileHeader,
    ImageImportDescriptor, ImageNtHeaders, ImageOptionalHeader, ImageSectionHeader, PE32,
    TlsDirectory32, IMAGE_DIRECTORY_ENTRY_DELAY_LOAD, IMAGE_DIRECTORY_ENTRY_EXPORT,
    IMAGE_DIRECTORY_ENTRY_IAT, IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_DIRECTORY_ENTRY_TLS,
    SECTION_HEADER_SZ,
};

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}

impl PE32 {
    /// True if `raw` is a 32-bit (x86) PE image, by its COFF `Machine` field.
    pub fn is_pe32(raw: &[u8]) -> bool {
        crate::pe::shared::pe_machine_type(raw)
            == Some(crate::pe::shared::IMAGE_FILE_MACHINE_I386)
    }

    pub fn read_string(raw: &[u8], off: usize) -> String {
        read_c_string(raw, off)
    }

    pub fn read_string_200(raw: &[u8], off: usize) -> String {
        read_c_string_with_max(raw, off, 200)
    }

    /// Parse PE32 metadata from `raw`. Does **not** retain `raw`.
    pub fn parse(filename: &str, raw: &[u8]) -> PE32 {
        let dos = ImageDosHeader::load(raw, 0);
        let nt = ImageNtHeaders::load(raw, dos.e_lfanew as usize);
        let fh = ImageFileHeader::load(raw, dos.e_lfanew as usize + 4);
        let opt = ImageOptionalHeader::load(raw, dos.e_lfanew as usize + 24);
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
            let mut delay_load_off = PE32::vaddr_to_off(&sect, delay_load_va) as usize;
            if delay_load_off > 0 {
                loop {
                    let mut delay_load = DelayLoadDirectory::load(raw, delay_load_off);
                    if delay_load.handle == 0 || delay_load.name_ptr == 0 {
                        break;
                    }
                    delay_load.name = read_c_string(raw, off);
                    delay_load_dir.push(delay_load);
                    delay_load_off += DelayLoadDirectory::size();
                }
            }
        }

        if import_va > 0 {
            let mut import_off = PE32::vaddr_to_off(&sect, import_va) as usize;
            if import_off > 0 {
                loop {
                    let mut iid = ImageImportDescriptor::load(raw, import_off);
                    if iid.name_ptr == 0 {
                        break;
                    }
                    let off = PE32::vaddr_to_off(&sect, iid.name_ptr) as usize;
                    if off > raw.len() {
                        panic!("the name of pe32 iid is out of buffer");
                    }
                    let libname = read_c_string(raw, off);
                    if libname.is_empty() {
                        import_off += ImageImportDescriptor::size();
                        continue;
                    }
                    iid.name = libname;
                    image_import_descriptor.push(iid);
                    import_off += ImageImportDescriptor::size();
                }
            }
        }

        let _exportd: Option<ImageExportDirectory> = if export_va > 0 { None } else { None };

        PE32 {
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

    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn mem_size(&self) -> usize {
        let mut max_va: u32 = 0;
        let mut max_va_sz: usize = 0;
        for sect in &self.sect_hdr {
            if sect.virtual_address > max_va {
                max_va = sect.virtual_address;
                if sect.size_of_raw_data > sect.virtual_size {
                    max_va_sz = sect.size_of_raw_data as usize;
                } else {
                    max_va_sz = sect.virtual_size as usize;
                }
            }
        }
        (max_va as usize) + max_va_sz
    }

    /// The PE headers slice (caller-owned bytes).
    pub fn headers<'a>(&self, raw: &'a [u8]) -> &'a [u8] {
        &raw[0..self.opt.size_of_headers as usize]
    }

    pub fn vaddr_to_off(sections: &Vec<ImageSectionHeader>, vaddr: u32) -> u32 {
        for sect in sections {
            if vaddr >= sect.virtual_address && vaddr < sect.virtual_address + sect.virtual_size {
                return vaddr - sect.virtual_address + sect.pointer_to_raw_data;
            }
        }
        0
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

    pub fn get_section_ptr<'a>(&self, raw: &'a [u8], id: usize) -> &'a [u8] {
        let off = self.sect_hdr[id].pointer_to_raw_data as usize;
        let mut sz = self.sect_hdr[id].size_of_raw_data as usize;
        if off + sz >= raw.len() {
            sz = raw.len() - off - 1;
        }
        if sz == 0 || off > raw.len() || off + sz > raw.len() {
            return &[];
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

        let tls_off = PE32::vaddr_to_off(&self.sect_hdr, entry_tls) as usize;
        let tls = TlsDirectory32::load(raw, tls_off);
        tls.print();

        if tls.tls_callbacks < self.opt.image_base - 0xf000 + 0xa400 {
            panic!("error loading tls callbacks");
        }
        let mut cb_off = (tls.tls_callbacks - self.opt.image_base - 0xf000 + 0xa400) as usize;

        loop {
            let callback: u64 = read_u32_le!(raw, cb_off) as u64;
            if callback == 0 {
                break;
            }
            log::trace!("TLS Callback: 0x{:x}", callback);
            callbacks.push(callback);
            cb_off += 4;
        }

        callbacks
    }
}
