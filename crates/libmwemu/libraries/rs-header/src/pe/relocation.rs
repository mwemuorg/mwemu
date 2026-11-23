use crate::pe::image::ImageLayout;
use crate::pe::readers::{read_u16_le, read_u32_le};
use crate::pe::shared::{
    IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_FILE_RELOCS_STRIPPED, IMAGE_REL_BASED_ABSOLUTE,
    IMAGE_REL_BASED_HIGHADJ, IMAGE_SIZEOF_FILE_HEADER, ImageDataDirectory, ImageFileHeader,
};

const BLOCK_HEADER_SIZE: usize = 8;
const DIRECTORY_ENTRY_SIZE: usize = 8;
const TYPE_SHIFT: u32 = 12;
const OFFSET_MASK: u16 = 0x0fff;

pub(super) fn directory<'a>(
    raw: &'a [u8],
    layout: ImageLayout<'_>,
    nt_offset: u32,
    file: &ImageFileHeader,
    directory_offset: usize,
    count: u32,
    directories: &[ImageDataDirectory],
) -> Option<&'a [u8]> {
    let optional_offset =
        (nt_offset as usize).checked_add(size_of::<u32>() + IMAGE_SIZEOF_FILE_HEADER as usize)?;
    let available = raw
        .get(optional_offset..)?
        .len()
        .min(file.size_of_optional_header as usize);
    if file.characteristics & IMAGE_FILE_RELOCS_STRIPPED != 0 || available < directory_offset {
        return None;
    }
    if count as usize <= IMAGE_DIRECTORY_ENTRY_BASERELOC {
        return Some(&[]);
    }
    let end = directory_offset + (IMAGE_DIRECTORY_ENTRY_BASERELOC + 1) * DIRECTORY_ENTRY_SIZE;
    if available < end {
        return None;
    }
    let directory = directories.get(IMAGE_DIRECTORY_ENTRY_BASERELOC)?;
    match (directory.virtual_address, directory.size) {
        (0, 0) => Some(&[]),
        (0, _) | (_, 0) => None,
        (rva, size) => layout.data_at(raw, rva)?.get(..size as usize),
    }
}

/// Visit every supported fixup. Report incomplete tables to automatic placement
/// while preserving usable fixups for callers that explicitly rebase an image.
pub(super) fn visit<const WIDTH: usize>(
    raw: &[u8],
    layout: ImageLayout<'_>,
    mut blocks: &[u8],
    expected_type: u16,
    mut apply: impl FnMut(u32, [u8; WIDTH]),
) -> Option<()> {
    let mut complete = true;
    while !blocks.is_empty() {
        let header = blocks.get(..BLOCK_HEADER_SIZE)?;
        let page = read_u32_le(header, 0);
        let size = read_u32_le(header, 4) as usize;
        let entries = blocks.get(BLOCK_HEADER_SIZE..size)?;
        let (entries, trailing) = entries.as_chunks::<2>();
        complete &= trailing.is_empty();
        let mut entries = entries.iter();
        while let Some(entry) = entries.next() {
            let encoded = read_u16_le(entry, 0);
            let kind = encoded >> TYPE_SHIFT;
            if kind == IMAGE_REL_BASED_ABSOLUTE {
                continue;
            }
            if kind != expected_type {
                complete = false;
                if kind == IMAGE_REL_BASED_HIGHADJ {
                    // The following WORD is an addend, not another fixup.
                    entries.next();
                }
                continue;
            }
            if let Some(rva) = page.checked_add((encoded & OFFSET_MASK) as u32)
                && let Some(value) = layout
                    .data_at(raw, rva)
                    .and_then(|data| data.first_chunk::<WIDTH>())
            {
                apply(rva, *value);
            } else {
                complete = false;
            }
        }
        blocks = &blocks[size..];
    }
    complete.then_some(())
}
