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
    let mut start = section.pointer_to_raw_data as u64;
    let mut end = start + section.size_of_raw_data as u64;
    if alignment >= PAGE_SIZE {
        start &= !(SECTOR_SIZE - 1);
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
