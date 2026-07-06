use crate::pe::readers::{read_u16_le as read_u16_le_shared, read_u32_le as read_u32_le_shared};
use crate::pe::structures;
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

impl PE64 {
    pub fn locate_resource_data_entry(
        &self,
        rsrc: &[u8],
        off: usize,
        level: u32,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<structures::ImageResourceDataEntry64> {
        if level >= 10 {
            log::warn!("Resource directory recursion limit reached");
            return None;
        }
        if off + 16 > rsrc.len() {
            log::warn!("Resource directory at offset {} out of bounds ({})", off, rsrc.len());
            return None;
        }

        let mut dir = structures::ImageResourceDirectory::new();
        dir.characteristics = read_u32_le!(rsrc, off);
        dir.time_date_stamp = read_u32_le!(rsrc, off + 4);
        dir.major_version = read_u16_le!(rsrc, off + 8);
        dir.minor_version = read_u16_le!(rsrc, off + 10);
        dir.number_of_named_entries = read_u16_le!(rsrc, off + 12);
        dir.number_of_id_entries = read_u16_le!(rsrc, off + 14);

        let entries = dir.number_of_named_entries + dir.number_of_id_entries;

        for i in 0..entries {
            let entry_off = off + (i as usize * 8) + 16;
            if entry_off + 8 > rsrc.len() {
                log::warn!("Resource directory entry {} at offset {} out of bounds", i, entry_off);
                continue;
            }

            let mut entry = structures::ImageResourceDirectoryEntry::new();
            entry.name_or_id = read_u32_le!(rsrc, entry_off);
            entry.data_or_directory = read_u32_le!(rsrc, entry_off + 4);

            let matched: bool;
            if entry.is_id() {
                let entry_id = entry.get_name_or_id();
                if level == 0 && type_id == Some(entry_id) {
                    matched = true;
                } else if level == 1 && name_id == Some(entry_id) {
                    matched = true;
                } else if level == 2 {
                    matched = true;
                } else {
                    matched = false;
                }
            } else {
                let name_offset = (entry.get_name_or_id() & 0x7FFFFFFF) as usize;
                if name_offset >= rsrc.len() {
                    continue;
                }
                let resource_name = self.read_resource_name_from_rsrc(rsrc, name_offset);
                if level == 0 && type_name == Some(resource_name.as_str()) {
                    matched = true;
                } else if level == 1 && name == Some(resource_name.as_str()) {
                    matched = true;
                } else {
                    matched = false;
                }
            }

            if matched {
                if entry.is_directory() {
                    let next_dir_offset = entry.get_offset() & 0x7FFFFFFF;
                    return self.locate_resource_data_entry(
                        rsrc,
                        next_dir_offset as usize,
                        level + 1,
                        type_id,
                        name_id,
                        type_name,
                        name,
                    );
                } else {
                    let data_entry_offset = entry.get_offset();
                    if data_entry_offset as usize + 16 > rsrc.len() {
                        return None;
                    }
                    let mut data_entry = structures::ImageResourceDataEntry64::new();
                    data_entry.offset_to_data = read_u32_le!(rsrc, data_entry_offset as usize) as u64;
                    data_entry.size = read_u32_le!(rsrc, data_entry_offset as usize + 4) as u64;
                    data_entry.code_page = read_u32_le!(rsrc, data_entry_offset as usize + 8) as u64;
                    data_entry.reserved = read_u32_le!(rsrc, data_entry_offset as usize + 12) as u64;
                    return Some(data_entry);
                }
            }
        }

        None
    }

    pub fn read_resource_name_from_rsrc(&self, rsrc: &[u8], offset: usize) -> String {
        if offset + 1 >= rsrc.len() {
            return String::new();
        }
        let length = u16::from_le_bytes([rsrc[offset], rsrc[offset + 1]]) as usize;
        let string_start = offset + 2;
        let required_bytes = string_start + (length * 2);
        if required_bytes > rsrc.len() {
            return String::new();
        }
        let utf16_data: Vec<u16> = (0..length)
            .map(|i| {
                let idx = string_start + i * 2;
                u16::from_le_bytes([rsrc[idx], rsrc[idx + 1]])
            })
            .collect();
        String::from_utf16_lossy(&utf16_data)
    }

    pub fn get_resource(
        &self,
        raw: &[u8],
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        let rsrc = self.get_section_ptr_by_name(raw, ".rsrc")?;
        let data_entry =
            self.locate_resource_data_entry(rsrc, 0, 0, type_id, name_id, type_name, name)?;
        let data_off = PE64::vaddr_to_off(&self.sect_hdr, data_entry.offset_to_data as u32) as usize
            - self.opt.image_base as usize;
        Some((data_off as u64, data_entry.size as usize))
    }

    pub fn get_resource_name(
        &self,
        raw: &[u8],
        entry: &structures::ImageResourceDirectoryEntry,
    ) -> String {
        let rsrc = match self.get_section_ptr_by_name(raw, ".rsrc") {
            Some(rsrc) => rsrc,
            None => return String::new(),
        };
        let name_offset = (entry.get_name_or_id() & 0x7FFFFFFF) as usize;
        self.read_resource_name_from_rsrc(rsrc, name_offset)
    }
}
