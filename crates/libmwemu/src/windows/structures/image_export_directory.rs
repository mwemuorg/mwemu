use crate::maps::Maps;

#[derive(Debug)]
pub struct ImageExportDirectory {
    characteristics: u32,
    time_date_stamp: u32,
    major_version: u16,
    minor_version: u16,
    name: u32,
    base: u32,
    number_of_functions: u32,
    number_of_names: u32,
    address_of_functions: u32,
    address_of_names: u32,
    address_of_ordinals: u32,
}

impl ImageExportDirectory {
    pub fn load(addr: u64, maps: &Maps) -> ImageExportDirectory {
        ImageExportDirectory {
            characteristics: maps.read_dword(addr).unwrap_or(0),
            time_date_stamp: maps.read_dword(addr + 4).unwrap_or(0),
            major_version: maps.read_word(addr + 8).unwrap_or(0),
            minor_version: maps.read_word(addr + 10).unwrap_or(0),
            name: maps.read_dword(addr + 12).unwrap_or(0),
            base: maps.read_dword(addr + 16).unwrap_or(0),
            number_of_functions: maps.read_dword(addr + 20).unwrap_or(0),
            number_of_names: maps.read_dword(addr + 24).unwrap_or(0),
            address_of_functions: maps.read_dword(addr + 28).unwrap_or(0),
            address_of_names: maps.read_dword(addr + 32).unwrap_or(0),
            address_of_ordinals: maps.read_dword(addr + 36).unwrap_or(0),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
