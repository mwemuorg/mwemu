use crate::emu::Emu;
use crate::winapi::common::sections;

pub fn MapViewOfFile(emu: &mut Emu) {
    sections::map_view(emu, false);
}
