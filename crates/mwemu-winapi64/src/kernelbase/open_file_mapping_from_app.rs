use crate::emu::Emu;
use crate::winapi::common::sections;

pub fn OpenFileMappingFromApp(emu: &mut Emu) {
    sections::open_file_mapping(emu, true);
}
