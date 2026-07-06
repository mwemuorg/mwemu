use crate::pe::structures;
use super::PE32;

macro_rules! read_u16_le {
    ($raw:expr, $off:expr) => {
        u16::from_le_bytes([$raw[$off], $raw[$off + 1]])
    };
}
macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        u32::from_le_bytes([$raw[$off], $raw[$off + 1], $raw[$off + 2], $raw[$off + 3]])
    };
}

impl PE32 {
    pub fn locate_resource_data_entry(
        &self,
        raw: &[u8],
        rsrc: &[u8],
        off: usize,
        level: u32,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<structures::ImageResourceDataEntry32> {
        if level >= 10 {
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
            let mut entry = structures::ImageResourceDirectoryEntry::new();
            let off2 = off + i as usize * 8 + structures::ImageResourceDirectory::size() as usize;
            entry.name_or_id = read_u32_le!(rsrc, off2);
            entry.data_or_directory = read_u32_le!(rsrc, off2 + 4);

            let matched: bool;
            if entry.is_id() {
                if level == 0 && type_id == Some(entry.get_name_or_id()) {
                    matched = true;
                } else if level == 1 && name_id == Some(entry.get_name_or_id()) {
                    matched = true;
                } else {
                    matched = level > 1;
                }
            } else if level == 0 && type_name == Some(self.get_resource_name(raw, &entry).as_str()) {
                matched = true;
            } else if level == 1 && name == Some(self.get_resource_name(raw, &entry).as_str()) {
                matched = true;
            } else {
                matched = level > 1;
            }

            if matched {
                if entry.is_directory() {
                    return self.locate_resource_data_entry(
                        raw, rsrc, off2, level + 1, type_id, name_id, type_name, name,
                    );
                } else {
                    let mut data_entry = structures::ImageResourceDataEntry32::new();
                    let doff = PE32::vaddr_to_off(&self.sect_hdr, entry.get_offset()) as usize;
                    data_entry.offset_to_data = read_u32_le!(raw, doff);
                    data_entry.size = read_u32_le!(raw, doff + 4);
                    data_entry.code_page = read_u32_le!(raw, doff + 8);
                    data_entry.reserved = read_u32_le!(raw, doff + 12);
                    return Some(data_entry);
                }
            }
        }

        None
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
            self.locate_resource_data_entry(raw, rsrc, 0, 0, type_id, name_id, type_name, name)?;
        let data_off = PE32::vaddr_to_off(&self.sect_hdr, data_entry.offset_to_data as u32) as usize
            - self.opt.image_base as usize;
        Some((data_off as u64, data_entry.size as usize))
    }

    pub fn get_resource_name(
        &self,
        raw: &[u8],
        entry: &structures::ImageResourceDirectoryEntry,
    ) -> String {
        let off = PE32::vaddr_to_off(&self.sect_hdr, entry.get_name_or_id()) as usize;
        if off + 1 >= raw.len() {
            return String::new();
        }
        let length = u16::from_le_bytes([raw[off], raw[off + 1]]) as usize;
        let string_start = off + 2;
        if string_start + length * 2 > raw.len() {
            return String::new();
        }
        let utf16_data: Vec<u16> = (0..length)
            .map(|i| {
                let idx = string_start + i * 2;
                u16::from_le_bytes([raw[idx], raw[idx + 1]])
            })
            .collect();
        String::from_utf16_lossy(&utf16_data)
    }
}
