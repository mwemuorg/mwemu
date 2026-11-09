use crate::emu::Emu;
use crate::winapi::common::sections;

pub fn UnmapViewOfFile(emu: &mut Emu) {
    sections::unmap_view(emu, false);
}
