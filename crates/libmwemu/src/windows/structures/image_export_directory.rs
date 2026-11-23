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
    pub(crate) const NUMBER_OF_NAMES_OFFSET: u64 = 0x18;
    pub(crate) const ADDRESS_OF_FUNCTIONS_OFFSET: u64 = 0x1c;
    pub(crate) const ADDRESS_OF_NAMES_OFFSET: u64 = 0x20;
    pub(crate) const ADDRESS_OF_ORDINALS_OFFSET: u64 = 0x24;

    const TIME_DATE_STAMP_OFFSET: u64 = 0x04;
    const MAJOR_VERSION_OFFSET: u64 = 0x08;
    const MINOR_VERSION_OFFSET: u64 = 0x0a;
    const NAME_OFFSET: u64 = 0x0c;
    const BASE_OFFSET: u64 = 0x10;
    const NUMBER_OF_FUNCTIONS_OFFSET: u64 = 0x14;

    pub fn load(addr: u64, maps: &Maps) -> ImageExportDirectory {
        ImageExportDirectory {
            characteristics: maps.read_dword(addr).unwrap_or(0),
            time_date_stamp: maps
                .read_dword(addr + Self::TIME_DATE_STAMP_OFFSET)
                .unwrap_or(0),
            major_version: maps
                .read_word(addr + Self::MAJOR_VERSION_OFFSET)
                .unwrap_or(0),
            minor_version: maps
                .read_word(addr + Self::MINOR_VERSION_OFFSET)
                .unwrap_or(0),
            name: maps.read_dword(addr + Self::NAME_OFFSET).unwrap_or(0),
            base: maps.read_dword(addr + Self::BASE_OFFSET).unwrap_or(0),
            number_of_functions: maps
                .read_dword(addr + Self::NUMBER_OF_FUNCTIONS_OFFSET)
                .unwrap_or(0),
            number_of_names: maps
                .read_dword(addr + Self::NUMBER_OF_NAMES_OFFSET)
                .unwrap_or(0),
            address_of_functions: maps
                .read_dword(addr + Self::ADDRESS_OF_FUNCTIONS_OFFSET)
                .unwrap_or(0),
            address_of_names: maps
                .read_dword(addr + Self::ADDRESS_OF_NAMES_OFFSET)
                .unwrap_or(0),
            address_of_ordinals: maps
                .read_dword(addr + Self::ADDRESS_OF_ORDINALS_OFFSET)
                .unwrap_or(0),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
