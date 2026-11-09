use crate::emu::Emu;
use crate::winapi::common::sections;

pub fn CreateFileMappingA(emu: &mut Emu) {
    sections::create_file_mapping(emu, false);
}
