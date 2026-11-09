use crate::emu::Emu;
use crate::winapi::common::sections;

pub fn UnmapViewOfFileEx(emu: &mut Emu) {
    sections::unmap_view(emu, true);
}
