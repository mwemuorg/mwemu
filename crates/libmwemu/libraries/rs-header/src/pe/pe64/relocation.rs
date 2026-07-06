use crate::pe::loader::PeLoader;
use crate::pe::readers::{
    read_u16_le as read_u16_le_shared, read_u32_le as read_u32_le_shared,
    read_u64_le as read_u64_le_shared,
};
use super::PE64;

macro_rules! read_u16_le {
    ($raw:expr, $off:expr) => {
        read_u16_le_shared(($raw).as_ref(), $off)
    };
}
macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}
macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

impl PE64 {
    /// Apply ADDR64 base relocations: read the un-relocated value from the file
    /// image `raw`, add the load delta, and patch the result into guest memory
    /// via `loader` (the section was mapped verbatim from `raw`, so the original
    /// value matches).
    pub fn apply_relocations<L: PeLoader>(&self, raw: &[u8], loader: &mut L, base_addr: u64) {
        if self.opt.data_directory.len() <= crate::pe::pe32::IMAGE_DIRECTORY_ENTRY_BASERELOC {
            return;
        }
        let reloc_dir = &self.opt.data_directory[crate::pe::pe32::IMAGE_DIRECTORY_ENTRY_BASERELOC];
        let reloc_va = reloc_dir.virtual_address;
        let reloc_sz = reloc_dir.size;

        if reloc_va == 0 || reloc_sz == 0 {
            return;
        }

        let delta = base_addr.wrapping_sub(self.opt.image_base);
        if delta == 0 {
            return;
        }

        let mut off = PE64::vaddr_to_off(&self.sect_hdr, reloc_va) as usize;
        if off == 0 {
            return;
        }

        let end_off = off + reloc_sz as usize;
        log::debug!("applying base relocations (delta 0x{:x})...", delta);

        while off < end_off && off + 8 <= raw.len() {
            let page_va = read_u32_le!(raw, off);
            let block_sz = read_u32_le!(raw, off + 4);

            if page_va == 0 && block_sz == 0 {
                break;
            }
            if block_sz < 8 {
                break;
            }

            let entries_count = (block_sz - 8) / 2;
            let mut entry_off = off + 8;

            for _ in 0..entries_count {
                if entry_off + 2 > raw.len() {
                    break;
                }
                let entry = read_u16_le!(raw, entry_off);
                let reloc_type = entry >> 12;
                let reloc_offset = entry & 0x0FFF;

                if reloc_type == 10 {
                    let target_rva = page_va + reloc_offset as u32;
                    let target_off = PE64::vaddr_to_off(&self.sect_hdr, target_rva) as usize;

                    if target_off > 0 && target_off + 8 <= raw.len() {
                        let original_val = read_u64_le!(raw, target_off);
                        let new_val = original_val.wrapping_add(delta);
                        let patch_addr = base_addr + target_rva as u64;
                        loader.write_qword(patch_addr, new_val);
                    }
                }
                entry_off += 2;
            }

            off += block_sz as usize;
        }

        log::debug!("base relocations applied.");
    }
}
