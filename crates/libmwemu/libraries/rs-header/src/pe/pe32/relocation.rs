use super::PE32;
use crate::pe::loader::PeLoader;
use crate::pe::readers::{read_u16_le, read_u32_le};
use crate::pe::shared::IMAGE_DIRECTORY_ENTRY_BASERELOC;

const BLOCK_HEADER_SIZE: usize = 8;
const RELOCATION_OFFSET_MASK: u16 = 0x0fff;
const RELOCATION_TYPE_SHIFT: u32 = 12;
const RELOCATION_HIGHLOW: u16 = 3;

impl PE32 {
    /// Rebase initialized image values through the same layout used to map them.
    pub fn apply_relocations<L: PeLoader>(&self, raw: &[u8], loader: &mut L, base_addr: u32) {
        let Some(directory) = self.opt.data_directory.get(IMAGE_DIRECTORY_ENTRY_BASERELOC) else {
            return;
        };
        if directory.virtual_address == 0 || directory.size == 0 {
            return;
        }
        let delta = base_addr.wrapping_sub(self.opt.image_base);
        if delta == 0 {
            return;
        }
        let layout = self.image_layout();
        let Some(data) = layout.data_at(raw, directory.virtual_address) else {
            return;
        };
        let Some(mut blocks) = data.get(..directory.size as usize) else {
            return;
        };
        log::trace!("applying base relocations (delta 0x{:x})...", delta);
        while blocks.len() >= BLOCK_HEADER_SIZE {
            let page = read_u32_le(blocks, 0);
            let size = read_u32_le(blocks, 4) as usize;
            if size < BLOCK_HEADER_SIZE || size > blocks.len() {
                break;
            }
            for entry in blocks[BLOCK_HEADER_SIZE..size].chunks_exact(size_of::<u16>()) {
                let entry = read_u16_le(entry, 0);
                if entry >> RELOCATION_TYPE_SHIFT != RELOCATION_HIGHLOW {
                    continue;
                }
                let Some(rva) = page.checked_add((entry & RELOCATION_OFFSET_MASK) as u32) else {
                    continue;
                };
                let Some(value) = layout
                    .data_at(raw, rva)
                    .and_then(|data| data.get(..size_of::<u32>()))
                else {
                    continue;
                };
                let Some(address) = (base_addr as u64).checked_add(rva as u64) else {
                    continue;
                };
                loader.write_dword(address, read_u32_le(value, 0).wrapping_add(delta));
            }
            blocks = &blocks[size..];
        }
        log::trace!("base relocations applied.");
    }
}
