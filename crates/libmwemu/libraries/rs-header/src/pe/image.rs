//! Windows image-section layout, distinct from literal raw-file section access.

use crate::pe::shared::ImageSectionHeader;

const PAGE_SIZE: u32 = 0x1000;
const SECTOR_SIZE: u64 = 0x200;

/// Round a 32-bit PE size without overflowing its field width.
/// Invalid alignment values are treated as unaligned, like absent headers.
pub fn aligned_size(size: u32, alignment: u32) -> u64 {
    let alignment = if alignment.is_power_of_two() {
        alignment as u64
    } else {
        1
    };
    (size as u64 + alignment - 1) & !(alignment - 1)
}

pub fn section_size(section: &ImageSectionHeader, alignment: u32) -> u64 {
    let size = if section.virtual_size == 0 {
        section.size_of_raw_data
    } else {
        section.virtual_size
    };
    aligned_size(size, alignment)
}

/// Include gaps and rounded section tails, even if SizeOfImage understates them.
pub fn image_size(
    headers: u32,
    declared: u32,
    sections: &[ImageSectionHeader],
    alignment: u32,
) -> u64 {
    sections.iter().fold(
        aligned_size(headers, alignment).max(declared as u64),
        |size, section| size.max(section.virtual_address as u64 + section_size(section, alignment)),
    )
}

/// Initialized bytes of a mapped section. Windows rounds ordinary image data
/// to 512-byte sectors independently of FileAlignment. Low-alignment images
/// retain their file offsets. A literal zero raw pointer denotes zero-fill;
/// a nonzero pointer rounded down to zero can instead map the file headers.
pub fn section_data<'a>(raw: &'a [u8], section: &ImageSectionHeader, alignment: u32) -> &'a [u8] {
    if section.pointer_to_raw_data == 0 || section.size_of_raw_data == 0 {
        return &[];
    }
    let start = section_offset(section, alignment);
    let mut end = section.pointer_to_raw_data as u64 + section.size_of_raw_data as u64;
    if alignment >= PAGE_SIZE {
        end = (end + SECTOR_SIZE - 1) & !(SECTOR_SIZE - 1);
    }
    end = end
        .min(start + section_size(section, alignment))
        .min(raw.len() as u64);
    if start >= end {
        return &[];
    }
    &raw[start as usize..end as usize]
}

/// File offset used when mapping an initialized image section.
fn section_offset(section: &ImageSectionHeader, alignment: u32) -> u64 {
    let offset = section.pointer_to_raw_data as u64;
    if alignment >= PAGE_SIZE {
        offset & !(SECTOR_SIZE - 1)
    } else {
        offset
    }
}

/// Literal raw-file access, independent of virtual size and image alignment.
pub fn raw_section_data<'a>(raw: &'a [u8], section: &ImageSectionHeader) -> &'a [u8] {
    let start = (section.pointer_to_raw_data as usize).min(raw.len());
    let size = (section.size_of_raw_data as usize).min(raw.len() - start);
    &raw[start..start + size]
}

/// Shared file-backed view used by PE metadata readers and the image loader.
#[derive(Clone, Copy)]
pub struct ImageLayout<'a> {
    pub headers: u32,
    pub size: u64,
    pub sections: &'a [ImageSectionHeader],
    pub alignment: u32,
}

impl<'a> ImageLayout<'a> {
    pub fn new(
        headers: u32,
        declared_size: u32,
        sections: &'a [ImageSectionHeader],
        alignment: u32,
    ) -> Self {
        Self {
            headers,
            size: image_size(headers, declared_size, sections, alignment),
            sections,
            alignment,
        }
    }

    /// Low-alignment images own the final page, but only SizeOfImage bytes
    /// are initialized from the file; the remaining page tail is zero-filled.
    pub fn mapped_size(&self) -> u64 {
        if self.is_flat() {
            self.size.div_ceil(PAGE_SIZE as u64) * PAGE_SIZE as u64
        } else {
            self.size
        }
    }

    pub fn is_flat(&self) -> bool {
        self.alignment.is_power_of_two() && self.alignment < PAGE_SIZE
    }

    /// Contiguous initialized bytes at an RVA. Zero-fill and unmapped gaps have
    /// no file offset; callers must not substitute offset zero for a failed lookup.
    pub fn data_at<'b>(&self, raw: &'b [u8], rva: u32) -> Option<&'b [u8]> {
        let rva = rva as u64;
        if self.is_flat() {
            let end = self.size.min(raw.len() as u64);
            return (rva < end).then(|| &raw[rva as usize..end as usize]);
        }
        if rva < self.headers as u64 {
            return raw.get(rva as usize..(self.headers as usize).min(raw.len()));
        }
        for section in self.sections {
            let Some(offset) = rva.checked_sub(section.virtual_address as u64) else {
                continue;
            };
            if offset < section_size(section, self.alignment) {
                let data = section_data(raw, section, self.alignment);
                return (offset < data.len() as u64).then(|| &data[offset as usize..]);
            }
        }
        None
    }
}
