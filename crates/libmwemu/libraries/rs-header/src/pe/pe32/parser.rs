use super::{
    DelayLoadDirectory, IMAGE_DIRECTORY_ENTRY_DELAY_LOAD, IMAGE_DIRECTORY_ENTRY_IMPORT,
    IMAGE_DIRECTORY_ENTRY_TLS, IMAGE_FILE_DLL, ImageDosHeader, ImageFileHeader,
    ImageImportDescriptor, ImageNtHeaders, ImageOptionalHeader, ImageSectionHeader, PE32,
    SECTION_HEADER_SZ, TlsDirectory32,
};
use crate::pe::image::{ImageLayout, raw_section_data};
use crate::pe::readers::{
    read_c_string, read_c_string_with_max, read_u32_le as read_u32_le_shared,
};

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}

impl PE32 {
    /// True if `raw` is a 32-bit (x86) PE image, by its COFF `Machine` field.
    pub fn is_pe32(raw: &[u8]) -> bool {
        crate::pe::shared::pe_machine_type(raw) == Some(crate::pe::shared::IMAGE_FILE_MACHINE_I386)
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
        let delay_load_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_DELAY_LOAD].virtual_address;

        let mut image_import_descriptor: Vec<ImageImportDescriptor> = Vec::new();
        let mut delay_load_dir: Vec<DelayLoadDirectory> = Vec::new();

        let layout = ImageLayout::new(
            opt.size_of_headers,
            opt.size_of_image,
            &sect,
            opt.section_alignment,
        );
        if delay_load_va != 0
            && let Some(data) = layout.data_at(raw, delay_load_va)
        {
            for descriptor in data.chunks_exact(DelayLoadDirectory::size()) {
                let mut delay = DelayLoadDirectory::load(descriptor, 0);
                if delay.name_ptr == 0 {
                    break;
                }
                let Some(name) = layout.data_at(raw, delay.name_ptr) else {
                    continue;
                };
                delay.name = read_c_string(name, 0);
                if !delay.name.is_empty() {
                    delay_load_dir.push(delay);
                }
            }
        }
        if import_va != 0
            && let Some(data) = layout.data_at(raw, import_va)
        {
            for descriptor in data.chunks_exact(ImageImportDescriptor::size()) {
                let mut import = ImageImportDescriptor::load(descriptor, 0);
                if import.name_ptr == 0 {
                    break;
                }
                let Some(name) = layout.data_at(raw, import.name_ptr) else {
                    continue;
                };
                import.name = read_c_string(name, 0);
                if !import.name.is_empty() {
                    image_import_descriptor.push(import);
                }
            }
        }

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

    /// The available PE header bytes, capped at the end of the file.
    pub fn headers<'a>(&self, raw: &'a [u8]) -> &'a [u8] {
        &raw[..(self.opt.size_of_headers as usize).min(raw.len())]
    }

    /// True if the COFF `Characteristics` field has the IMAGE_FILE_DLL bit set.
    pub fn is_dll(&self) -> bool {
        self.fh.characteristics & IMAGE_FILE_DLL != 0
    }

    /// Section-only raw-file lookup. Image metadata should use image_layout().
    pub fn vaddr_to_off(sections: &Vec<ImageSectionHeader>, vaddr: u32) -> u32 {
        for section in sections {
            if let Some(offset) = vaddr.checked_sub(section.virtual_address)
                && offset < section.virtual_size
            {
                return section.pointer_to_raw_data.checked_add(offset).unwrap_or(0);
            }
        }
        0
    }

    pub fn image_layout(&self) -> ImageLayout<'_> {
        ImageLayout::new(
            self.opt.size_of_headers,
            self.opt.size_of_image,
            &self.sect_hdr,
            self.opt.section_alignment,
        )
    }

    pub fn num_of_sections(&self) -> usize {
        self.sect_hdr.len()
    }

    pub fn get_section_ptr_by_name<'a>(&self, raw: &'a [u8], name: &str) -> Option<&'a [u8]> {
        self.sect_hdr
            .iter()
            .find(|section| section.get_name() == name)
            .map(|section| raw_section_data(raw, section))
    }

    pub fn get_section(&self, id: usize) -> &ImageSectionHeader {
        &self.sect_hdr[id]
    }

    pub fn get_section_ptr<'a>(&self, raw: &'a [u8], id: usize) -> &'a [u8] {
        self.sect_hdr
            .get(id)
            .map(|section| raw_section_data(raw, section))
            .unwrap_or_default()
    }

    pub fn get_section_vaddr(&self, id: usize) -> u32 {
        self.sect_hdr[id].virtual_address
    }

    pub fn get_tls_callbacks(&self, raw: &[u8], _vaddr: u32) -> Vec<u64> {
        let Some(tls) = self.get_tls_directory(raw) else {
            return Vec::new();
        };
        tls.print();
        if tls.tls_callbacks == 0 {
            return Vec::new();
        }
        let Some(rva) = tls.tls_callbacks.checked_sub(self.opt.image_base) else {
            return Vec::new();
        };
        let Some(data) = self.image_layout().data_at(raw, rva) else {
            return Vec::new();
        };
        data.chunks_exact(4)
            .take(64)
            .map(|entry| read_u32_le!(entry, 0) as u64)
            .take_while(|&callback| callback != 0 && callback >= self.opt.image_base as u64)
            .collect()
    }

    pub fn get_tls_directory(&self, raw: &[u8]) -> Option<TlsDirectory32> {
        let entry = self.opt.data_directory.get(IMAGE_DIRECTORY_ENTRY_TLS)?;
        if entry.virtual_address == 0 {
            return None;
        }
        let data = self.image_layout().data_at(raw, entry.virtual_address)?;
        let data = data.get(..24)?;
        Some(TlsDirectory32::load(data, 0))
    }
}
